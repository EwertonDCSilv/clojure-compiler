(ns io.clojure_edn.read_string.boundary)
(defn -main [] (do (clojure.edn/read-string {:readers {} :default (fn [tag value] [tag value])} "#app/id 42") (println :ok)))
(-main)
