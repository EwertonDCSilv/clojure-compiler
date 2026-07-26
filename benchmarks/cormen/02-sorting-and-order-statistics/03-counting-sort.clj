(ns cormen.sorting.counting-sort)

(defn frequencies [values]
  (loop [i 0 counts [0 0 0 0 0 0 0 0 0 0]]
    (if (< i (count values))
      (let [value (nth values i)]
        (recur (inc i) (assoc counts value (inc (nth counts value)))))
      counts)))

(defn counting-sort [values]
  (let [counts (frequencies values)]
    (loop [value 0 remaining (nth counts 0) result []]
      (cond
        (>= value 10) result
        (> remaining 0)
        (recur value (dec remaining) (conj result value))
        :else
        (let [next-value (inc value)]
          (if (< next-value 10)
            (recur next-value (nth counts next-value) result)
            result))))))

(defn weighted-sum [values]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (* (inc i) (nth values i))))
      total)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (weighted-sum
                  (counting-sort [7 2 5 3 7 1 0 9 4 2 8 5 6 3 1]))))
      checksum)))

(defn -main [] (println (benchmark 8000)))
(-main)
