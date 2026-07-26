(ns c.mapv)
(defn -main [] (println (mapv inc (range 3)) (mapv inc (list)) (mapv (fn [x] (* x 2)) (list -1 0 1))))
(-main)
