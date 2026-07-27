(ns io.clojure_core.prn.error)
(defn -main []
(try
(do (binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (prn 1)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
