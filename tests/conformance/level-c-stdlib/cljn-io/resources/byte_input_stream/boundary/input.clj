(ns io.cljn_io.byte_input_stream.boundary)
(defn -main [] (do (cljn.io/close! (cljn.io/byte-input-stream (cljn.io/bytes []))) (println :ok)))
(-main)
