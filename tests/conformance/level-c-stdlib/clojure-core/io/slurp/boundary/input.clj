(ns io.clojure_core.slurp.boundary)
(defn -main [] (do (slurp "empty.txt") (println :ok)))
(-main)
