(ns io.cljn_io.list.error)
(defn -main []
(try
(do (cljn.io/list "input.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
