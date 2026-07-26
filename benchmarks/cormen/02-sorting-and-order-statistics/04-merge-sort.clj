(ns cormen.sorting.merge-sort)

(defn take-prefix [values amount]
  (loop [i 0 result []]
    (if (< i amount)
      (recur (inc i) (conj result (nth values i)))
      result)))

(defn drop-prefix [values amount]
  (loop [i amount result []]
    (if (< i (count values))
      (recur (inc i) (conj result (nth values i)))
      result)))

(defn merge-vectors [left right]
  (loop [i 0 j 0 result []]
    (cond
      (>= i (count left))
      (if (>= j (count right))
        result
        (recur i (inc j) (conj result (nth right j))))
      (>= j (count right))
      (recur (inc i) j (conj result (nth left i)))
      (<= (nth left i) (nth right j))
      (recur (inc i) j (conj result (nth left i)))
      :else
      (recur i (inc j) (conj result (nth right j))))))

(defn merge-sort [values]
  (if (<= (count values) 1)
    values
    (let [middle (quot (count values) 2)]
      (merge-vectors
        (merge-sort (take-prefix values middle))
        (merge-sort (drop-prefix values middle))))))

(defn weighted-sum [values]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (* (+ i 5) (nth values i))))
      total)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (weighted-sum
                  (merge-sort [42 7 19 3 31 12 5 27 1 36 14 9 23 2]))))
      checksum)))

(defn -main [] (println (benchmark 3500)))
(-main)
