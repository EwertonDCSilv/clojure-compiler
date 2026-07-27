(ns io.cljn_io.write_bytes_.normal)
(defn -main [] (do (cljn.io/write-bytes! (cljn.io/byte-output-stream) (cljn.io/bytes [0 255])) (println :ok)))
(-main)
