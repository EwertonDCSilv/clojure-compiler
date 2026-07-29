;; JVM Pedestal HTTP benchmark server (ADR-0013 Gate 6).
;;
;; Mirrors ../../native/greet_server.clj: a single minimal route,
;; GET /greet -> 200 "Hello, world!\n", served by the pinned upstream Pedestal
;; http-kit connector. Default interceptors are intentionally omitted so the
;; server exposes the same P1 capability level as the native connector (route
;; match, handler, response serialization) instead of Pedestal's additional
;; security-header and content-negotiation stack; the comparison then isolates
;; dispatch and serialization rather than optional middleware.
;;
;; The listen port is taken from the first command-line argument so the
;; orchestrator can reserve a free port up front. Readiness is detected by the
;; orchestrator polling the port, matching how it waits on the native server.
(ns greet-server
  (:require [io.pedestal.connector :as conn]
            [io.pedestal.http.http-kit :as hk]))

(defn greet-handler
  "Return the fixed greeting; unmatched paths fall through to Pedestal's 404."
  [_request]
  {:status 200 :body "Hello, world!\n"})

(def routes
  #{["/greet" :get greet-handler :route-name :greet]})

(defn -main [& args]
  (let [port (Integer/parseInt (or (first args) "8890"))]
    (-> (conn/default-connector-map port)
        (conn/with-routes routes)
        (hk/create-connector nil)
        (conn/start!))
    (println (str "listening " port))
    (flush)
    @(promise)))
