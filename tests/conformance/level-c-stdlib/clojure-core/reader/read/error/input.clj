(ns io.clojure_core.read.error)
(defn -main []
(try
(do (read (cljn.io/string-reader "[1")) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
