(ns c.filter)
(defn -main [] (println (filter even? (range 6)) (filter even? (list)) (filter (fn [x] (> x 2)) (list 1 3 4))))
(-main)
