(ns b.capture-gc)
(defn make [x] (fn [y] (list x y)))
(defn -main [] (println ((make (list 1 2)) (list 3 4))))
(-main)
