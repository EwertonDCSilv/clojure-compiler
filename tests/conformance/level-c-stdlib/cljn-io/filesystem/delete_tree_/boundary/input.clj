(ns io.cljn_io.delete_tree_.boundary)
(defn -main [] (do (cljn.io/delete-tree! "missing" :missing :ignore) (println :ok)))
(-main)
