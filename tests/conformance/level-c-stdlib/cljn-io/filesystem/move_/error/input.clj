(ns io.cljn_io.move_.error)
(defn -main []
(try
(do (cljn.io/move! "missing.txt" "moved.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
