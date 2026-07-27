(ns hello
  (:require [io.pedestal.connector :as conn]
            [io.pedestal.http.http-kit :as hk]))

(defn greet-handler [_request]
  {:status 200
   :body "Hello, world!\n"})

(def routes
  #{["/greet" :get greet-handler :route-name :greet]})

(defn create-connector []
  (-> (conn/default-connector-map 8890)
      (conn/with-default-interceptors)
      (conn/with-routes routes)
      (hk/create-connector nil)))

(defn start []
  (conn/start! (create-connector)))

(defn -main [& _args]
  (start))
