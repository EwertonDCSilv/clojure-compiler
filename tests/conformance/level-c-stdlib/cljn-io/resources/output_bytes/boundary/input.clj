(ns io.cljn_io.output_bytes.boundary)
(defn -main [] (do (cljn.io/output-bytes (cljn.io/byte-output-stream)) (println :ok)))
(-main)
