(ns b.cond)
(defn sign [x] (cond (< x 0) :neg (= x 0) :zero :else :pos))
(defn -main [] (println (sign -1) (sign 0) (sign 1)))
(-main)
