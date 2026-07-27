(ns io.cljn_io.input_stream.boundary)
(defn -main [] (do (cljn.io/close! (cljn.io/input-stream "empty.txt")) (println :ok)))
(-main)
