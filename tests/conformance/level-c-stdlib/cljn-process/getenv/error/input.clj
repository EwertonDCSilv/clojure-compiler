(ns io.cljn_process.getenv.error)
(defn -main []
(try
(do (cljn.process/getenv nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
