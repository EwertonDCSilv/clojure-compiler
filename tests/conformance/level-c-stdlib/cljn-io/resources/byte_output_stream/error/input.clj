(ns io.cljn_io.byte_output_stream.error)
(defn -main []
(try
(do (cljn.io/byte-output-stream -1) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
