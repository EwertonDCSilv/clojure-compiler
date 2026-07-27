(ns io.cljn_io.output_bytes.error)
(defn -main []
(try
(do (cljn.io/output-bytes (cljn.io/input-stream "input.txt")) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
