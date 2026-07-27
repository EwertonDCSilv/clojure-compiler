(ns b.io.unwind-binding)
(defn -main []
  (try (binding [*out* (cljn.io/string-writer)] (throw (ex-info "boom" {})))
       (catch Exception error (println "restored"))))
(-main)
