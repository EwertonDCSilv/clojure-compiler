(ns io.cljn_io.absolute_.error)
(defn -main []
(try
(do (cljn.io/absolute? "relative") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
