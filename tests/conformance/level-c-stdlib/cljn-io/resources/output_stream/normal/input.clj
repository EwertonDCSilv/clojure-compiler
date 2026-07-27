(ns io.cljn_io.output_stream.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/output-stream "created.bin")) (println :ok)))
(-main)
