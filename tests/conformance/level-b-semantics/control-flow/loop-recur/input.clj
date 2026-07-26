(ns b.loop)
(defn sum [n] (loop [i 0 acc 0] (if (> i n) acc (recur (inc i) (+ acc i)))))
(defn -main [] (println (sum 0) (sum 10) (sum 100)))
(-main)
