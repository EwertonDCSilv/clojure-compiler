(ns io.clojure_core.read_string.normal)
(defn -main [] (do (read-string "{:answer 42}") (println :ok)))
(-main)
