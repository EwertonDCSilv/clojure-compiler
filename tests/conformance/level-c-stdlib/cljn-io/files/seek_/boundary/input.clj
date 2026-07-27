(ns io.cljn_io.seek_.boundary)
(defn -main [] (do (with-open [s (cljn.io/input-stream "empty.txt")] (cljn.io/seek! s 0)) (println :ok)))
(-main)
