(ns native-http-server
  (:require [cljn.http.response :as response]
            [cljn.pedestal.connector :as connector]
            [cljn.pedestal.service :as service]))

(def dump-path "target/last-request-body.json")

(defn handler
  "Logs requests, persists POST /dump bodies, and exposes a health endpoint."
  [request]
  (println "request:" (get request :method) (get request :path))
  (flush)
  (if (and (= :post (get request :method))
           (= "/dump" (get request :path)))
    (let [body (get request :body)]
      (spit dump-path body)
      (println "body dump:")
      (println body)
      (println "saved:" dump-path)
      (flush)
      (response/with-header
        (response/ok "{\"saved\":\"target/last-request-body.json\"}\n")
        :content-type
        "application/json"))
    (if (= "/health" (get request :path))
      (response/with-header
        (response/ok "up\n")
        :content-type
        "text/plain")
      (response/not-found))))

(defn -main
  "Starts the synchronous native HTTP server on loopback port 18080."
  []
  (let [running (service/start!
                  (connector/create-connector handler)
                  {:port 18080})]
    (println "listening: http://127.0.0.1:18080")
    (flush)
    (service/serve! running)))

(-main)
