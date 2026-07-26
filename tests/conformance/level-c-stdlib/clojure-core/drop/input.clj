(ns c.drop)
(defn -main [] (println (drop 2 (range 5)) (drop 0 (range 3)) (drop 9 (list 1 2))))
(-main)
