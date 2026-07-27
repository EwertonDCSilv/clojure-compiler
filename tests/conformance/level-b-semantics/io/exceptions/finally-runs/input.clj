(ns b.io.finally)
(defn -main []
  (try (throw (ex-info "boom" {}))
       (catch Exception error (print "caught "))
       (finally (println "closed"))))
(-main)
