(ns io.cljn_io.parent.normal)
(defn -main [] (do (cljn.io/parent (cljn.io/path "tree/a.txt")) (println :ok)))
(-main)
