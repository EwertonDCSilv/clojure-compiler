(ns io.cljn_io.bytes__vector.error)
(defn -main []
(try
(do (cljn.io/bytes->vector nil) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
