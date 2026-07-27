(ns io.cljn_io.symlink_.error)
(defn -main []
(try
(do (cljn.io/symlink? nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
