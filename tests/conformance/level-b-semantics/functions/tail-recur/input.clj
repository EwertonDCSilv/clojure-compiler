(ns b.tail)
(defn count-down [n acc] (if (= n 0) acc (recur (dec n) (inc acc))))
(defn -main [] (println (count-down 0 0) (count-down 10 0) (count-down 10000 0)))
(-main)
