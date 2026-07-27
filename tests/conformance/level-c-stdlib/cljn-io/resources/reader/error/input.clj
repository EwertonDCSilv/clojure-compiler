(ns io.cljn_io.reader.error)
(defn -main []
(try
(do (cljn.io/reader "missing.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
