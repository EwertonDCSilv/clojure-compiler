;; cljn.pedestal.connector — conector síncrono P0/P1 (ADR-0013).
;;
;; Módulo built-in de propriedade do compilador. Preserva um subconjunto
;; documentado de conceitos do Pedestal, mas NÃO compila o Pedestal upstream nem
;; implementa o protocolo PedestalConnector (ADR-0013 §1). test-request e o caminho
;; de rede compartilham a mesma função de handler (ADR-0013 §2, aceitação #3).
(ns cljn.pedestal.connector)

(defn create-connector
  "Cria um conector a partir de um `handler` (fn requisição->resposta). Devolve um
  mapa imutável; não abre servidor de rede (Planned: start!/serve!, ADR-0013 §3).
  A cadeia de interceptors e o roteamento entram em gates posteriores (§10)."
  [handler]
  {:handler handler})

(defn test-request
  "Despacha a requisição `req` pela mesma função de handler que o caminho de rede
  usaria e devolve a resposta. Não abre sockets. Construção/execução puras."
  [connector req]
  ((get connector :handler) req))
