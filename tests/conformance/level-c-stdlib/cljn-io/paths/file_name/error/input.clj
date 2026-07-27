(ns io.cljn_io.file_name.error)
(defn -main []
(try
(do (cljn.io/file-name nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
