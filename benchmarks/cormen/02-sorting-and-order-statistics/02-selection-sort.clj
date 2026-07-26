(ns cormen.sorting.selection-sort)

(defn minimum-index [values start]
  (loop [i (inc start) best start]
    (if (>= i (count values))
      best
      (recur (inc i)
             (if (< (nth values i) (nth values best)) i best)))))

(defn swap-at [values left right]
  (let [a (nth values left)
        b (nth values right)]
    (assoc (assoc values left b) right a)))

(defn selection-sort [values]
  (loop [i 0 result values]
    (if (>= i (dec (count result)))
      result
      (let [best (minimum-index result i)]
        (recur (inc i) (swap-at result i best))))))

(defn weighted-sum [values]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (* (+ i 3) (nth values i))))
      total)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (weighted-sum
                  (selection-sort [44 12 5 38 1 29 17 8 33 3 21 14]))))
      checksum)))

(defn -main [] (println (benchmark 5000)))
(-main)
