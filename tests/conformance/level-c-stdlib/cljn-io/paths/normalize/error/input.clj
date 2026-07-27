(ns io.cljn_io.normalize.error)
(defn -main []
(try
(do (cljn.io/normalize 42) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
