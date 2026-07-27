(ns io.cljn_io.output_bytes.normal)
(defn -main [] (do (let [out (cljn.io/byte-output-stream)] (cljn.io/write-bytes! out (cljn.io/bytes [0 255])) (cljn.io/output-bytes out)) (println :ok)))
(-main)
