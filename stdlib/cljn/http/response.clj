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
