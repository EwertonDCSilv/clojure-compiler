(ns io.cljn_io.read_link.error)
(defn -main []
(try
(do (cljn.io/read-link "input.txt") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
