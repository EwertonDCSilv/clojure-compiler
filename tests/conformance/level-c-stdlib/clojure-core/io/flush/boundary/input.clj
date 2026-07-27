(ns io.clojure_core.flush.boundary)
(defn -main [] (do (binding [*out* (cljn.io/string-writer)] (flush)) (println :ok)))
(-main)
