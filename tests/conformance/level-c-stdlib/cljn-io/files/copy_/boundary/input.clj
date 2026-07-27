(ns io.cljn_io.copy_.boundary)
(defn -main [] (do (cljn.io/copy! "empty.txt" "empty-copy.txt") (println :ok)))
(-main)
