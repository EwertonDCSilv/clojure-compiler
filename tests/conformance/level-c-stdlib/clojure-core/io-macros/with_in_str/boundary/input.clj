(ns io.clojure_core.with_in_str.boundary)
(defn -main [] (do (with-in-str "" (read-line)) (println :ok)))
(-main)
