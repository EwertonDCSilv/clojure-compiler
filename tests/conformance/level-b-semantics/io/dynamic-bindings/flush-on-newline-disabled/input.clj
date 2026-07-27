(ns b.io.flush-binding)
(defn -main []
  (binding [*flush-on-newline* false] (println "buffered") (flush)))
(-main)
