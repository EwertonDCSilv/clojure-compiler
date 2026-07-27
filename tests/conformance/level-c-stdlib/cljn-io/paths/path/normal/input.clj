(ns io.cljn_io.path.normal)
(defn -main [] (do (cljn.io/path "tree/nested/b.txt") (println :ok)))
(-main)
