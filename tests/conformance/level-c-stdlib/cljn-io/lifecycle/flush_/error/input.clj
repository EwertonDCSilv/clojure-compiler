(ns io.cljn_io.flush_.error)
(defn -main []
(try
(do (cljn.io/flush! (doto (cljn.io/string-writer) cljn.io/close!)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
