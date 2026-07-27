(ns io.clojure_core.spit.error)
(defn -main []
(try
(do (spit "tree" "not-a-file") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
