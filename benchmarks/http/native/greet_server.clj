;; Native HTTP benchmark server (ADR-0013 Gate 6).
;;
;; Serves a single minimal route, GET /greet -> 200 "Hello, world!\n", over the
;; compiled cljn.pedestal.* connector and the native loopback HTTP provider. The
;; response body and status are byte-for-byte identical to the JVM Pedestal server
;; in ../jvm/src/greet_server.clj, so the load client can checksum both and prove
;; they serve equivalent content before any timing is compared.
;;
;; The process prints its ephemeral port on the first stdout line and then blocks
;; in the synchronous accept loop until it receives SIGINT/SIGTERM, at which point
;; it shuts down cleanly with no leaked descriptor. The orchestrator binds the
;; port from that first line and stops the server with a signal.
(ns greet-server
  (:require [cljn.http.response :as resp]
            [cljn.pedestal.connector :as conn]
            [cljn.pedestal.service :as svc]))

(defn handler
  "Return the fixed greeting for /greet and 404 for every other path."
  [request]
  (if (= (get request :path) "/greet")
    (resp/ok "Hello, world!\n")
    (resp/not-found)))

(defn -main []
  (let [running (svc/start! (conn/create-connector handler) {:port 0})]
    (println (svc/server-port running))
    (flush)
    (svc/serve! running)))

(-main)
