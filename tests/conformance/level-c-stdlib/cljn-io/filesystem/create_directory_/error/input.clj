(ns io.cljn_io.create_directory_.error)
(defn -main []
(try
(do (cljn.io/create-directory! "tree") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
