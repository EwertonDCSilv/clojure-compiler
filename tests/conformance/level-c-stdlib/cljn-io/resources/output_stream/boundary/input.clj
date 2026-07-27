(ns io.cljn_io.output_stream.boundary)
(defn -main [] (do (cljn.io/close! (cljn.io/output-stream "created.bin" :append true)) (println :ok)))
(-main)
