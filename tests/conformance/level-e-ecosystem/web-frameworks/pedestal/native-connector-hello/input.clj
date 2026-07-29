;; ADR-0013 Gate 5: native Pedestal connector Hello World over test-request.
;;
;; Exercises the P1 in-memory dispatch path only: a router-backed connector
;; threads a request map through the interceptor chain and the deterministic
;; router to a handler response. It imports the compiler-owned cljn.pedestal.*
;; namespaces and uses no io.pedestal, HTTP-Kit, Maven, or network transport,
;; so it runs offline without a JVM. Network lifecycle is covered separately by
;; the CLI/runtime integration tests.
(ns hello.native-connector
  (:require [cljn.http.request :as req]
            [cljn.http.response :as resp]
            [cljn.pedestal.connector :as conn]))

(defn greet
  "Router handler returning a greeting built from the captured :name segment."
  [request]
  (resp/ok (str "Hello, " (get (get request :path-params) :name) "!")))

(defn -main []
  (let [c (conn/create-router-connector
            [{:segments ["hello"] :method :get :name :root
              :handler (fn [_] (resp/ok "Hello, World!"))}
             {:segments ["hello" :name] :method :get :name :greet :handler greet}])]
    (println (get (conn/test-request c (req/request :get "/hello")) :body))
    (println (get (conn/test-request c (req/request :get "/hello/Ada")) :body))
    (println (get (conn/test-request c (req/request :get "/hello")) :status))
    (println (get (conn/test-request c (req/request :get "/missing")) :status))
    (println (get (conn/test-request c (req/request :post "/hello")) :status))))

(-main)
