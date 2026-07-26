(ns c.mapcat)
(defn pair [x] (list x x))
(defn -main [] (println (mapcat pair (list 1 2)) (mapcat pair (list)) (mapcat (fn [x] (list (inc x))) (list 0 1))))
(-main)
