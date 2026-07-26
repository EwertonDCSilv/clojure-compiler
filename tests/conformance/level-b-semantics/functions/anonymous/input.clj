(ns b.anon)
(defn -main [] (println ((fn [x] (+ x 1)) 4) ((fn [x y] (* x y)) 6 7)))
(-main)
