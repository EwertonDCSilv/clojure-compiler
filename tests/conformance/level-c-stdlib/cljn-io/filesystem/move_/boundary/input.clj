(ns io.cljn_io.move_.boundary)
(defn -main [] (do (cljn.io/move! "empty.txt" "moved-empty.txt") (println :ok)))
(-main)
