(ns c.take)
(defn -main [] (println (take 2 (range 5)) (take 0 (range 5)) (take 9 (list 1 2))))
(-main)
