(ns io.cljn_io.parent.error)
(defn -main []
(try
(do (cljn.io/parent "not-a-path") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
