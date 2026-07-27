(ns io.cljn_io.exists_.error)
(defn -main []
(try
(do (cljn.io/exists? nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
