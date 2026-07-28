;; cljn.http.request — construtores e acessores de requisição HTTP (conector P1).
;;
;; Módulo built-in de propriedade do compilador (ADR-0013). Uma requisição é um
;; mapa imutável {:method keyword :path string :headers mapa :body nil|string}.
;; A mesma forma é usada por test-request e pelo caminho de rede (ADR-0013 §10).
(ns cljn.http.request)

(defn request
  "Constrói uma requisição com `method` (keyword, ex.: :get) e `path` (string),
  com `body` opcional (nil por padrão) e `:headers` vazio. Construção pura,
  não-preguiçosa; sem validação semântica ainda (Planned: ADR-0013 §10)."
  ([method path] (request method path nil))
  ([method path body] {:method method :path path :headers {} :body body}))

(defn method
  "Método da requisição (keyword)."
  [req]
  (get req :method))

(defn path
  "Caminho da requisição (string)."
  [req]
  (get req :path))
