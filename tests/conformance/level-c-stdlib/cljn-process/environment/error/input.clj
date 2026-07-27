(ns io.cljn_process.environment.error)
(defn -main []
(try
(do (cljn.process/environment :unexpected) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
