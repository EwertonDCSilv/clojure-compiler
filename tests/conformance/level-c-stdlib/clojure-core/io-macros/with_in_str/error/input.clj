(ns io.clojure_core.with_in_str.error)
(defn -main []
(try
(do (with-in-str nil (read-line)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
