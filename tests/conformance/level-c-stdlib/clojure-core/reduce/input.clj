(ns c.reduce)
(defn -main [] (println (reduce + 0 (list 1 2 3)) (reduce + 10 (list)) (reduce * 1 [1 2 3 4])))
(-main)
