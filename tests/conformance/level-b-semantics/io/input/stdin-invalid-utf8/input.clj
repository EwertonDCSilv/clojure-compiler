(ns b.io.invalid-utf8)
(defn -main []
  (println
    (try
      (do (read-line) :unexpected)
      (catch E error (get error :kind)))))
(-main)
