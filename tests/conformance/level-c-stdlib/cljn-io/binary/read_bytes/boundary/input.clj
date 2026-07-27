(ns io.cljn_io.read_bytes.boundary)
(defn -main [] (do (cljn.io/read-bytes (cljn.io/byte-input-stream (cljn.io/bytes [])) 0) (println :ok)))
(-main)
