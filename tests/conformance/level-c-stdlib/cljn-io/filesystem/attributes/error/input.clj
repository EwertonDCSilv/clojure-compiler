(ns io.cljn_io.attributes.error)
(defn -main []
(try
(do (cljn.io/attributes "missing.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
