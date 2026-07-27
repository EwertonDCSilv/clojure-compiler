(ns io.cljn_io.create_directories_.error)
(defn -main []
(try
(do (cljn.io/create-directories! "input.txt/child") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
