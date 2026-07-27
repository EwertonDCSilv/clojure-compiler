(ns io.cljn_io.closed_.error)
(defn -main []
(try
(do (cljn.io/closed? 42) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
