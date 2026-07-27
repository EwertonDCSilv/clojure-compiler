(ns io.cljn_io.read_bytes.error)
(defn -main []
(try
(do (cljn.io/read-bytes (cljn.io/string-reader "x") 1) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
