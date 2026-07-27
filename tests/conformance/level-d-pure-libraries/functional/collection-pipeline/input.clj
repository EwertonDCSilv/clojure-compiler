(ns d.functional-pipeline)
(defn transform [xs]
(mapv (fn [x] (+ (* x x) 1)) (filter odd? xs)))
(defn -main [] (println (transform (range 8))))
(-main)
