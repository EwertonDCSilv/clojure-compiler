(ns io.clojure_core.flush.error)
(defn -main []
(try
(do (binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (flush)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
