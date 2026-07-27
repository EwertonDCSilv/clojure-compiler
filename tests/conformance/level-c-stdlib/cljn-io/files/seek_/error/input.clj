(ns io.cljn_io.seek_.error)
(defn -main []
(try
(do (with-open [s (cljn.io/input-stream "input.txt")] (cljn.io/seek! s -1)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
