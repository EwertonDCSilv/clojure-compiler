;; cljn.http.response — construtores de resposta HTTP para o conector nativo P1.
;;
;; Módulo built-in de propriedade do compilador (ADR-0013). Uma resposta é um mapa
;; imutável {:status inteiro :headers mapa :body nil|string}. Este subconjunto P0/P1
;; não implementa o PedestalConnector upstream; ver ADR-0013 §1.
(ns cljn.http.response)

(defn respond
  "Constrói uma resposta com um `status` inteiro e um `body` opcional (nil por
  padrão). `:headers` começa vazio. Sem validação semântica ainda (Planned:
  ADR-0013 §10); é uma construção pura, não-preguiçosa."
  ([status] (respond status nil))
  ([status body] {:status status :headers {} :body body}))

(defn ok
  "Resposta 200 com `body`. Construção pura."
  [body]
  (respond 200 body))

(defn not-found
  "Resposta 404 com corpo textual padrão. Construção pura."
  []
  (respond 404 "Not Found"))

(defn with-header
  "Devolve a resposta com o header `k`->`v` adicionado (persistente; não muta a
  entrada). `k` deve ser uma keyword de nome de header em minúsculas."
  [response k v]
  (assoc response :headers (assoc (get response :headers) k v)))

(defn response-valid?
  "Verdadeiro se `x` é um mapa de resposta bem-formado: `:status` inteiro em
  100..599, `:headers` mapa, `:body` nil ou string. Predicado puro. (Nome com
  prefixo `response-` até o compilador ter espaços de nomes por-namespace.)"
  [x]
  (and (map? x)
       (int? (get x :status))
       (>= (get x :status) 100)
       (<= (get x :status) 599)
       (map? (get x :headers))
       (let [b (get x :body)] (or (nil? b) (string? b) (bytes? b)))))

(defn content-length
  "Comprimento do corpo em bytes: 0 para nil; `count` para string (bytes UTF-8) ou
  Bytes. Pura."
  [response]
  (let [b (get response :body)]
    (if (nil? b) 0 (count b))))

(defn validate-response
  "Devolve `x` se for uma resposta válida; senão lança um mapa de erro
  categorizado (ADR-0013 §7): {:cljn.error/domain :http :kind :invalid-response
  :operation :validate-response :value x}. Construção pura; não muta."
  [x]
  (if (response-valid? x)
    x
    (throw {:cljn.error/domain :http
            :kind :invalid-response
            :operation :validate-response
            :value x})))
