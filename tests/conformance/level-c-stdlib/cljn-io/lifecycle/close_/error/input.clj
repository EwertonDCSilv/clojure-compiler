(ns io.cljn_io.close_.error)
(defn -main []
(try
(do (cljn.io/close! *out*) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
