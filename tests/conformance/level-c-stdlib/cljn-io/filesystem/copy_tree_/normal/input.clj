(ns io.cljn_io.copy_tree_.normal)
(defn -main [] (do (cljn.io/copy-tree! "tree" "tree-copy") (println :ok)))
(-main)
