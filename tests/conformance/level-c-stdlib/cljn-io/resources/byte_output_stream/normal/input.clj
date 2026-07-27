(ns io.cljn_io.byte_output_stream.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/byte-output-stream)) (println :ok)))
(-main)
