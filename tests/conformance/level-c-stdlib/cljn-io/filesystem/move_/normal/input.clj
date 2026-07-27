(ns io.cljn_io.move_.normal)
(defn -main [] (do (cljn.io/move! "input.txt" "moved.txt") (println :ok)))
(-main)
