(ns io.clojure_core.with_in_str.boundary)
(defn -main [] (println (nil? (with-in-str "" (read-line)))))
(-main)
