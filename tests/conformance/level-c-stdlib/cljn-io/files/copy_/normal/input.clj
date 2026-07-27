(ns io.cljn_io.copy_.normal)
(defn -main [] (do (cljn.io/copy! "input.txt" "copy.txt") (println :ok)))
(-main)
