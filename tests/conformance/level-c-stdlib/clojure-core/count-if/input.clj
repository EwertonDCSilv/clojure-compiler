(ns c.count-if)
(defn -main [] (println (count-if even? (range 6)) (count-if even? (list)) (count-if neg? (list -2 -1 0 1))))
(-main)
