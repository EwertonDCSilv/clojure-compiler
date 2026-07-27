(ns io.cljn_io.string_writer.error)
(defn -main []
(try
(do (cljn.io/string-writer -1) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
