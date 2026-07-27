(ns io.cljn_io.exists_.normal)
(defn -main [] (do (cljn.io/exists? "input.txt") (println :ok)))
(-main)
