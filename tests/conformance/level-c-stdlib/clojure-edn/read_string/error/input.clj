(ns io.clojure_edn.read_string.error)
(defn -main []
(try
(do (clojure.edn/read-string "#=(+ 1 2)") (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
