(ns io.cljn_io.normalize.normal)
(defn -main [] (do (cljn.io/normalize (cljn.io/path "tree/../input.txt")) (println :ok)))
(-main)
