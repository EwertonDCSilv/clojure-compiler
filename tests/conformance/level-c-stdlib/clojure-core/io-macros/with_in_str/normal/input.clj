(ns io.clojure_core.with_in_str.normal)
(defn -main [] (do (with-in-str "alpha\n" (read-line)) (println :ok)))
(-main)
