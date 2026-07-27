(ns io.cljn_io.create_symlink_.error)
(defn -main []
(try
(do (cljn.io/create-symlink! "input.txt" "input.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
