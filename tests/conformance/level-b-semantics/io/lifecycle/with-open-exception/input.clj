(ns b.io.with-open-exception)
(defn -main []
  (let [r (cljn.io/string-reader "x")]
    (try (with-open [opened r] (throw (ex-info "boom" {})))
         (catch Exception error (println (cljn.io/closed? r))))))
(-main)
