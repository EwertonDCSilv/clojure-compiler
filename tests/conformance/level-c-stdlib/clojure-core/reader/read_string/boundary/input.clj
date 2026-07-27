(ns io.clojure_core.read_string.boundary)
(defn -main [] (do (read-string "nil") (println :ok)))
(-main)
