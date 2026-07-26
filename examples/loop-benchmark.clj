(ns bench.core)
(defn -main []
  (println (loop [i 0 acc 0]
             (if (< i 100000000)
               (recur (inc i) (+ acc 1))
               acc))))
(-main)
