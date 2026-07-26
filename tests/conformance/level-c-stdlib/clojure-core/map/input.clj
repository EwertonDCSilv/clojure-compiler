(ns c.map)
(defn -main [] (println (map inc (list 1 2 3)) (map inc (list)) (map (fn [x] (* x x)) [2 3])))
(-main)
