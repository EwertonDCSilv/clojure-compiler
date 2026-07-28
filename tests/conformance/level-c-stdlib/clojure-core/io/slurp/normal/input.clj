(ns io.clojure_core.slurp.normal)
(defn -main [] (println (= "alpha\nbeta\n" (slurp "input.txt"))))
(-main)
