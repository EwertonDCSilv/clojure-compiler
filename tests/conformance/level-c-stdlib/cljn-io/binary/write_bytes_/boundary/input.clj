(ns io.cljn_io.write_bytes_.boundary)
(defn -main [] (do (cljn.io/write-bytes! (cljn.io/byte-output-stream) (cljn.io/bytes [])) (println :ok)))
(-main)
