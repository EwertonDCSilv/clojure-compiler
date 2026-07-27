(ns io.cljn_io.bytes__string.error)
(defn -main []
(try
(do (cljn.io/bytes->string (cljn.io/bytes [128])) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
