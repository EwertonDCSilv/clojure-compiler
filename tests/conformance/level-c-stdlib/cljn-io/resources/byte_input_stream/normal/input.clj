(ns io.cljn_io.byte_input_stream.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/byte-input-stream (cljn.io/bytes [1 2]))) (println :ok)))
(-main)
