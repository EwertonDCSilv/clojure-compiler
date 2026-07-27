(ns d.variadic-api)
(defn total [x & xs] (reduce + x xs))
(defn -main []
(println (total 1) (total 1 2 3 4) (apply total [10 20 12])))
(-main)
