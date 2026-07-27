(ns io.cljn_io.writer_string.error)
(defn -main []
(try
(do (cljn.io/writer-string (cljn.io/string-reader "x")) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
