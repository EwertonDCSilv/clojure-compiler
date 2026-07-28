(ns io.clojure_core.read_string.boundary)
(defn -main [] (println (nil? (read-string "nil"))))
(-main)
