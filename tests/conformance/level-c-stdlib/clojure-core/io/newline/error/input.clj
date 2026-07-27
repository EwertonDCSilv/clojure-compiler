(ns io.clojure_core.newline.error)
(defn -main []
(try
(do (binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (newline)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
