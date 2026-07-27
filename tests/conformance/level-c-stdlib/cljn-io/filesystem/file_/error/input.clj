(ns io.cljn_io.file_.error)
(defn -main []
(try
(do (cljn.io/file? nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
