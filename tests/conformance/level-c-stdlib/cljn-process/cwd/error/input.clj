(ns io.cljn_process.cwd.error)
(defn -main []
(try
(do (cljn.process/cwd :unexpected) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
