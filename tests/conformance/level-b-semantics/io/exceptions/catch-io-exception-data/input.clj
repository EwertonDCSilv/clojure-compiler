(ns b.io.catch)
(defn -main []
  (try (slurp "missing.txt")
       (catch cljn.io/IOException error
         (println (:kind (ex-data error)) (:operation (ex-data error))))))
(-main)
