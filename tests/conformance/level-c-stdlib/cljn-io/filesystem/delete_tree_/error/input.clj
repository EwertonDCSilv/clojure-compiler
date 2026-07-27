(ns io.cljn_io.delete_tree_.error)
(defn -main []
(try
(do (cljn.io/delete-tree! "/") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
