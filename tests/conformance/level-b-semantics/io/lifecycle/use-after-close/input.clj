(ns b.io.use-after-close)
(defn -main []
  (let [r (cljn.io/string-reader "x")]
    (cljn.io/close! r)
    (try (cljn.io/read-char r)
         (catch cljn.io/IOException error (println (:kind (ex-data error)))))))
(-main)
