(ns b.gc-loop)
(defn burn [n] (loop [i 0] (if (< i n) (do (count (cons i (list))) (recur (inc i))) i)))
(defn -main [] (println (burn 20000)))
(-main)
