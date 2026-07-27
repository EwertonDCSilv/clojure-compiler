(ns io.cljn_io.write_bytes_.error)
(defn -main []
(try
(do (cljn.io/write-bytes! (cljn.io/string-writer) (cljn.io/bytes [1])) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
