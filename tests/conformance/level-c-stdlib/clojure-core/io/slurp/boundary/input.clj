(ns io.clojure_core.slurp.boundary)
(defn -main [] (println (= "" (slurp "empty.txt"))))
(-main)
