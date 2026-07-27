(ns io.cljn_io.parent.boundary)
(defn -main [] (do (cljn.io/parent (cljn.io/path "/")) (println :ok)))
(-main)
