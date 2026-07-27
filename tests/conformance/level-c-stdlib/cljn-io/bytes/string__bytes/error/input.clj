(ns io.cljn_io.string__bytes.error)
(defn -main []
(try
(do (cljn.io/string->bytes nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
