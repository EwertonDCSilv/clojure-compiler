(ns io.clojure_core.slurp.error)
(defn -main []
(try
(do (slurp "missing.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
