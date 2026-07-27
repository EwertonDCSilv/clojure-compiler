(ns io.cljn_io.bytes.error)
(defn -main []
(try
(do (cljn.io/bytes [256]) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
