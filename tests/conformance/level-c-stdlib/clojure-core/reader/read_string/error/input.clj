(ns io.clojure_core.read_string.error)
(defn -main []
(try
(do (read-string "#=(+ 1 2)") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
