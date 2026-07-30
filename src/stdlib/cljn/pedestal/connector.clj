;; cljn.pedestal.connector — conector síncrono P0/P1 (ADR-0013).
;;
;; Módulo built-in de propriedade do compilador. Preserva um subconjunto
;; documentado de conceitos do Pedestal, mas NÃO compila o Pedestal upstream nem
;; implementa o protocolo PedestalConnector (ADR-0013 §1). Um conector guarda uma
;; cadeia de interceptors; test-request e o caminho de rede a executam pelo MESMO
;; caminho (ADR-0013 §2, aceitação #3), validando a requisição na entrada e a
;; resposta na saída (§10). Sem sockets, threads ou async neste subconjunto P1.
(ns cljn.pedestal.connector
  (:require [cljn.http.request :as req]
            [cljn.http.response :as resp]
            [cljn.pedestal.chain :as chain]
            [cljn.pedestal.route :as route]))

(defn- handler-interceptor
  "Envolve um handler (fn requisição->resposta) como interceptor terminal cujo
  :enter fixa a resposta no contexto."
  [handler]
  {:name :cljn.connector/handler
   :enter (fn [ctx] (assoc ctx :response (handler (get ctx :request))))})

(defn create-connector
  "Conector a partir de um `handler` único (fn requisição->resposta). Lança um mapa
  de erro categorizado se `handler` for nil (ADR-0013 §7). Não abre rede."
  [handler]
  (if (nil? handler)
    (throw {:cljn.error/domain :http
            :kind :invalid-connector
            :operation :create-connector})
    {:interceptors [(handler-interceptor handler)]}))

(defn with-interceptors
  "Conector a partir de uma cadeia explícita de `interceptors` (vetor)."
  [interceptors]
  {:interceptors interceptors})

(defn create-router-connector
  "Conector cuja cadeia é o interceptor de roteamento para `routes` (ADR-0013 §10).
  Rotas ambíguas são rejeitadas na criação (via route/router)."
  [routes]
  {:interceptors [(route/router routes)]})

(defn test-request
  "Despacha a `request` pela cadeia de interceptors do conector — o mesmo caminho
  que a rede usaria. Valida a requisição na entrada e a resposta na saída, lançando
  mapas de erro categorizados em caso de forma inválida. Não abre sockets."
  [connector request]
  (let [ctx (chain/execute {:request (req/validate request)}
                           (get connector :interceptors))]
    (resp/validate (get ctx :response))))
