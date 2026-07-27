(ns io.cljn_io.delete_.normal)
(defn -main [] (do (cljn.io/delete! "input.txt") (println :ok)))
(-main)
