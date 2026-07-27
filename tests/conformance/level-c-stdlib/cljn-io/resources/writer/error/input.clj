(ns io.cljn_io.writer.error)
(defn -main []
(try
(do (cljn.io/writer "tree") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
