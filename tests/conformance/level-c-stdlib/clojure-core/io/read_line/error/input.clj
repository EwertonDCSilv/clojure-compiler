(ns io.clojure_core.read_line.error)
(defn -main []
(try
(do (binding [*in* (cljn.io/byte-input-stream (cljn.io/bytes [128]))] (read-line)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
