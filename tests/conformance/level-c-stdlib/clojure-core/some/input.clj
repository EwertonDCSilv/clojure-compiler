(ns c.some)
(defn -main [] (println (some even? (list 1 2 3)) (some even? (list 1 3)) (some (fn [x] (if (> x 2) x nil)) (list 1 3 4))))
(-main)
