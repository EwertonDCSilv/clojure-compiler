(ns io.cljn_io.position.error)
(defn -main []
(try
(do (cljn.io/position (cljn.io/string-reader "x")) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
