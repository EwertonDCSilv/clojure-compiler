(ns io.cljn_io.position.boundary)
(defn -main [] (do (with-open [s (cljn.io/input-stream "empty.txt")] (cljn.io/position s)) (println :ok)))
(-main)
