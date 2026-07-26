(ns c.remove)
(defn -main [] (println (remove even? (range 6)) (remove even? (list)) (remove neg? (list -1 0 1))))
(-main)
