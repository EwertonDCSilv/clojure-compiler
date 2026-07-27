(ns b.io.close-idempotent)
(defn -main []
  (let [r (cljn.io/string-reader "x")]
    (cljn.io/close! r)
    (cljn.io/close! r)
    (println (cljn.io/closed? r))))
(-main)
