(ns io.cljn_io.input_stream.error)
(defn -main []
(try
(do (cljn.io/input-stream "missing.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
