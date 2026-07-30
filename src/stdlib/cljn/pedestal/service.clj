;; cljn.pedestal.service — ciclo de vida síncrono do servidor P1 (ADR-0013 §3).
;;
;; Módulo built-in de propriedade do compilador. O loop de serviço em Clojure
;; conduz o provider de socket C: aceita uma requisição, despacha pela MESMA cadeia
;; de interceptors do conector (o mesmo caminho de test-request, aceitação #3) e
;; responde. `:join? false` NÃO cria execução em segundo plano — o chamador deve
;; invocar serve!/serve-one! (divergência single-thread deliberada, §3). Um serve!
;; bloqueante para com SIGINT/SIGTERM, um stop! no caminho de dispatch, ou erro do
;; provider. Sem threads, async ou preguiça neste subconjunto P1.
(ns cljn.pedestal.service
  (:require [cljn.http.request :as req]
            [cljn.http.response :as resp]
            [cljn.pedestal.chain :as chain]))

(defn- dispatch
  "Despacha `request` pela cadeia de interceptors do `running`, validando entrada e
  saída. Devolve a resposta."
  [running request]
  (let [ctx (chain/execute {:request (req/validate request)}
                           (get running :interceptors))]
    (resp/validate (get ctx :response))))

(defn serve-one!
  "Aceita e serve uma requisição. Devolve :stopped se um stop foi solicitado (sinal
  ou stop!), senão :served. Requisição malformada ou erro de dispatch viram uma
  resposta 400/500 na mesma conexão."
  [running]
  (let [srv (get running :server)]
    (try
      (let [request (http-server-accept srv)]
        (if (nil? request)
          :stopped
          (do (http-server-respond srv (dispatch running request)) :served)))
      (catch E e
        (http-server-respond srv {:status 400 :headers {} :body nil})
        :served))))

(defn serve!
  "Serve requisições até um stop (sinal, stop!, ou erro). Fecha o servidor ao sair e
  devolve `running`. Bloqueia a thread do chamador (single-thread, §3)."
  [running]
  (if (= :stopped (serve-one! running))
    (do (http-server-close (get running :server)) running)
    (recur running)))

(defn start!
  "Abre o listener na porta de `opts` (`:port`, 0 = efêmera) e devolve o `connector`
  em execução com o handle do servidor. Com `:join? true`, entra em serve! e só
  retorna no shutdown; com `:join? false` (padrão), devolve imediatamente e o
  chamador deve invocar serve!/serve-one! (§3)."
  [connector opts]
  (let [port (get opts :port)
        srv (http-server-open (if (nil? port) 0 port))
        running (assoc connector :server srv)]
    (if (get opts :join?)
      (serve! running)
      running)))

(defn server-port
  "A porta ligada do servidor em execução (útil após `:port` 0)."
  [running]
  (http-server-port (get running :server)))

(defn stop!
  "Solicita um shutdown gracioso: acorda um serve! bloqueado (via self-pipe) e faz
  a próxima iteração sair. Seguro a partir de um caminho de dispatch. Devolve
  `running`."
  [running]
  (http-server-stop (get running :server))
  running)
