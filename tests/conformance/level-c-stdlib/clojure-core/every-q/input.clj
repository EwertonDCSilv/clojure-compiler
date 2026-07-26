(ns c.every)
(defn -main [] (println (every? even? (list 2 4)) (every? even? (list)) (every? even? (list 2 3))))
(-main)
