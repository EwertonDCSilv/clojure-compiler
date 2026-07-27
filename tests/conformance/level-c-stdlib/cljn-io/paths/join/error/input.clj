(ns io.cljn_io.join.error)
(defn -main []
(try
(do (cljn.io/join nil "x") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
