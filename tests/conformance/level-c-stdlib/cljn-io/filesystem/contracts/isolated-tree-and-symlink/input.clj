(ns c.io.tree)
(defn -main []
  (cljn.io/copy-tree! "source" "copy")
  (cljn.io/delete-tree! "source")
  (println :ok))
(-main)
