(ns io.cljn_io.copy_.error)
(defn -main []
(try
(do (cljn.io/copy! "missing.txt" "copy.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
