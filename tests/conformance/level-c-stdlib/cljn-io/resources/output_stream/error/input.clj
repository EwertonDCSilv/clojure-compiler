(ns io.cljn_io.output_stream.error)
(defn -main []
(try
(do (cljn.io/output-stream "tree") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
