(ns io.cljn_io.copy_tree_.boundary)
(defn -main [] (do (cljn.io/copy-tree! "tree" "tree-copy" :overwrite true) (println :ok)))
(-main)
