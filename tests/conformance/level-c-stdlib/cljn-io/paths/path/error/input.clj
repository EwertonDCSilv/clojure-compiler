(ns io.cljn_io.path.error)
(defn -main []
(try
(do (cljn.io/path nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
