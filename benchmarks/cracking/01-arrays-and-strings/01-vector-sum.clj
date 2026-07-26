(ns cracking.arrays.vector-sum)

(defn vector-sum [xs]
  (loop [i 0 total 0]
    (if (< i (count xs))
      (recur (inc i) (+ total (nth xs i)))
      total)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n) (+ checksum (vector-sum [3 1 4 1 5 9 2 6])))
      checksum)))

(defn -main [] (println (benchmark 5000)))
(-main)
