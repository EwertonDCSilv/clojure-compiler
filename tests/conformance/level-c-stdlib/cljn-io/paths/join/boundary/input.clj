(ns io.cljn_io.join.boundary)
(defn -main [] (do (cljn.io/join (cljn.io/path ".")) (println :ok)))
(-main)
