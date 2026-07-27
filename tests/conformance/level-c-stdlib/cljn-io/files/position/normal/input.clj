(ns io.cljn_io.position.normal)
(defn -main [] (do (with-open [s (cljn.io/input-stream "input.txt")] (cljn.io/position s)) (println :ok)))
(-main)
