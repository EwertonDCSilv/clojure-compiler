(ns io.cljn_io.byte_input_stream.error)
(defn -main []
(try
(do (cljn.io/byte-input-stream "bytes") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
