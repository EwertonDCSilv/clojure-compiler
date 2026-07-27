(ns io.cljn_io.read_bytes.normal)
(defn -main [] (do (cljn.io/read-bytes (cljn.io/byte-input-stream (cljn.io/bytes [0 255])) 2) (println :ok)))
(-main)
