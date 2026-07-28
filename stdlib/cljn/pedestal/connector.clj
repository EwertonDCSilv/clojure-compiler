;; cljn.pedestal.connector — conector síncrono P0/P1 (ADR-0013).
;;
;; Módulo built-in de propriedade do compilador. Preserva um subconjunto
;; documentado de conceitos do Pedestal, mas NÃO compila o Pedestal upstream nem
;; implementa o protocolo PedestalConnector (ADR-0013 §1). test-request e o caminho
;; de rede compartilham a mesma função de handler (ADR-0013 §2, aceitação #3), e
;; ambos validam a requisição na entrada e a resposta na saída (§10).
(ns cljn.pedestal.connector
  (:require [cljn.http.request :as req]
            [cljn.http.response :as resp]))

(defn create-connector
  "Cria um conector a partir de um `handler` (fn requisição->resposta). Lança um
  mapa de erro categorizado se `handler` for nil (ADR-0013 §7). Devolve um mapa
  imutável; não abre servidor de rede (Planned: start!/serve!, ADR-0013 §3). A
  cadeia de interceptors e o roteamento entram em gates posteriores (§10)."
  [handler]
  (if (nil? handler)
    (throw {:cljn.error/domain :http
            :kind :invalid-connector
            :operation :create-connector})
    {:handler handler}))

(defn test-request
  "Despacha a `request` pela mesma função de handler que o caminho de rede usaria.
  Valida a requisição na entrada e a resposta na saída (lançando mapas de erro
  categorizados em caso de forma inválida). Não abre sockets. Sem preguiça."
  [connector request]
  (resp/validate
   ((get connector :handler) (req/validate request))))
