(ns io.cljn_io.read_char.error)
(defn -main []
(try
(do (cljn.io/read-char (doto (cljn.io/string-reader "x") cljn.io/close!)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
