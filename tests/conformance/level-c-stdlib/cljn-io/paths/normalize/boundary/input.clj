(ns io.cljn_io.normalize.boundary)
(defn -main [] (do (cljn.io/normalize (cljn.io/path ".")) (println :ok)))
(-main)
