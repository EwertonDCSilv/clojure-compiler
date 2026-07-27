(ns io.cljn_io.write_.error)
(defn -main []
(try
(do (cljn.io/write! (doto (cljn.io/string-writer) cljn.io/close!) "x") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
