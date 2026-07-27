(ns io.cljn_io.read_line.error)
(defn -main []
(try
(do (cljn.io/read-line (doto (cljn.io/string-reader "x") cljn.io/close!)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
