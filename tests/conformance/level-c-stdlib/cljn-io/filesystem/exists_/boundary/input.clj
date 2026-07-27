(ns io.cljn_io.exists_.boundary)
(defn -main [] (do (cljn.io/exists? "missing.txt") (println :ok)))
(-main)
