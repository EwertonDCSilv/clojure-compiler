(ns io.cljn_io.byte_count.error)
(defn -main []
(try
(do (cljn.io/byte-count "abc") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
