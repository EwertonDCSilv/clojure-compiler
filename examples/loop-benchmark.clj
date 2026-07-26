(ns benchmark.core)

(defn sum-range [n]
  (loop [i 0 acc 0]
    (if (> i n)
      acc
      (recur (inc i) (+ acc i)))))

(defn count-down [n]
  (loop [i n acc 0]
    (if (< i 0)
      acc
      (recur (dec i) (inc acc)))))

(defn bench [n times]
  (loop [t times]
    (if (<= t 0)
      (sum-range n)
      (do
        (sum-range n)
        (recur (dec t))))))

(defn -main []
  (println "sum 1..10000 =" (sum-range 10000))
  (println "count-down 2000000 =" (count-down 2000000))
  (println "bench 10000 x 30000 =" (bench 10000 30000)))

(-main)
