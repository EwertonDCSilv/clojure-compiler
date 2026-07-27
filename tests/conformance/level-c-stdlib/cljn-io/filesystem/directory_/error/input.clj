(ns io.cljn_io.directory_.error)
(defn -main []
(try
(do (cljn.io/directory? nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
