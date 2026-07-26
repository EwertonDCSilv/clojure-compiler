(ns cracking.arrays.reverse-vector)

(defn reverse-vector [xs]
  (loop [i (dec (count xs)) out []]
    (if (>= i 0)
      (recur (dec i) (conj out (nth xs i)))
      out)))

(defn weighted-checksum [xs]
  (loop [i 0 total 0]
    (if (< i (count xs))
      (recur (inc i) (+ total (* (inc i) (nth xs i))))
      total)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n) (+ checksum (weighted-checksum (reverse-vector [2 7 1 8 2 8]))))
      checksum)))

(defn -main [] (println (benchmark 2500)))
(-main)
