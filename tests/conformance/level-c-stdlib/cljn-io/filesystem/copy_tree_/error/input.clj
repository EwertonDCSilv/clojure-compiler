(ns io.cljn_io.copy_tree_.error)
(defn -main []
(try
(do (cljn.io/copy-tree! "/" "root-copy") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
