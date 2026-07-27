(ns io.cljn_io.input_stream.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/input-stream "input.txt")) (println :ok)))
(-main)
