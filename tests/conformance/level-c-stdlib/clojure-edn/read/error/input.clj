(ns io.clojure_edn.read.error)
(defn -main []
(try
(do (clojure.edn/read (cljn.io/string-reader "{:a")) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
