(ns io.cljn_io.string_reader.error)
(defn -main []
(try
(do (cljn.io/string-reader nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
