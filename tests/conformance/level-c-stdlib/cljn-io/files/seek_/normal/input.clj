(ns io.cljn_io.seek_.normal)
(defn -main [] (do (with-open [s (cljn.io/input-stream "input.txt")] (cljn.io/seek! s 2)) (println :ok)))
(-main)
