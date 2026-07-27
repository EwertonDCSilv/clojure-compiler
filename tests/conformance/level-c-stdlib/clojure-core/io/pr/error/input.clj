(ns io.clojure_core.pr.error)
(defn -main []
(try
(do (binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (pr 1)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
