;; cljn.pedestal.route — roteador linear determinístico (ADR-0013 §10).
;;
;; Módulo built-in de propriedade do compilador. Uma rota é um mapa
;; {:segments [seg...] :method :get :name :n :handler fn}, onde cada `seg` é uma
;; string literal ou uma keyword de captura de um segmento (:param). O roteamento é
;; um interceptor normal: seu :enter casa o path/método da requisição e produz a
;; resposta do handler, ou 404 (sem casamento de path) / 405 (path casa, método
;; não). Casamento linear O(rotas × segmentos); uma trie fica para depois de o
;; benchmark de rotas justificar (ADR-0013). Rotas duplicadas (mesmos segmentos e
;; método) são rejeitadas na criação.
(ns cljn.pedestal.route
  (:require [cljn.http.request :as req]))

(defn- path-segments
  "Segmentos não-vazios de um path absoluto: \"/a/b\" -> [\"a\" \"b\"], \"/\" -> [].
  Descarta as strings vazias produzidas pelas barras de borda/adjacentes."
  [path]
  (reduce (fn [acc s] (if (= s "") acc (conj acc s)))
          []
          (str-split path \/)))

(defn- match-segs
  "Casa os `route-segs` contra os `req-segs`, acumulando parâmetros capturados em
  `params`. Devolve o mapa de parâmetros no casamento, ou nil se não casar. Uma
  keyword em `route-segs` captura o segmento correspondente; uma string casa
  literalmente."
  [route-segs req-segs params]
  (if (empty? route-segs)
    (if (empty? req-segs) params nil)
    (if (empty? req-segs)
      nil
      (let [rs (first route-segs)
            qs (first req-segs)]
        (if (keyword? rs)
          (recur (rest route-segs) (rest req-segs) (assoc params rs qs))
          (if (= rs qs)
            (recur (rest route-segs) (rest req-segs) params)
            nil))))))

(defn- find-route
  "Varredura linear das `routes` contra os `segs` e o `method` da requisição a
  partir do índice `i`. Devolve {:route r :params p} no casamento; senão {:status
  405} se algum path casou com método divergente, ou {:status 404}."
  [routes segs method i n path-match]
  (if (>= i n)
    (if path-match {:status 405} {:status 404})
    (let [r (nth routes i)
          p (match-segs (get r :segments) segs {})]
      (if (nil? p)
        (recur routes segs method (inc i) n path-match)
        (if (= (get r :method) method)
          {:route r :params p}
          (recur routes segs method (inc i) n true))))))

(defn- dup?
  "Verdadeiro se `routes` contém duas rotas com mesmos :segments e :method."
  [routes i n]
  (if (>= i n)
    false
    (let [ri (nth routes i)]
      (if (dup-from? routes ri (inc i) n)
        true
        (recur routes (inc i) n)))))

(defn- dup-from?
  "Verdadeiro se alguma rota de `routes` a partir de `j` casa segmentos+método com
  `ref`."
  [routes ref j n]
  (if (>= j n)
    false
    (let [rj (nth routes j)]
      (if (and (= (get ref :segments) (get rj :segments))
               (= (get ref :method) (get rj :method)))
        true
        (recur routes ref (inc j) n)))))

(defn router
  "Constrói o interceptor de roteamento para `routes` (vetor de rotas). Lança um
  mapa de erro categorizado se houver rotas ambíguas (mesmos segmentos e método).
  O :enter casa a requisição e produz a resposta do handler, ou 404/405. Construção
  pura; sem preguiça."
  [routes]
  (if (dup? routes 0 (count routes))
    (throw {:cljn.error/domain :http
            :kind :ambiguous-routes
            :operation :router})
    {:name :cljn.route/router
     :enter (fn [ctx]
              (let [request (get ctx :request)
                    segs (path-segments (req/path request))
                    result (find-route routes segs (req/method request) 0 (count routes) false)
                    rt (get result :route)]
                (if (nil? rt)
                  (assoc ctx :response {:status (get result :status) :headers {} :body nil})
                  (let [handler (get rt :handler)
                        request2 (assoc request :path-params (get result :params))]
                    (assoc ctx :response (handler request2))))))}))
