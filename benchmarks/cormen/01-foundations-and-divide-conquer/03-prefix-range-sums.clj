(ns cormen.foundations.prefix-range-sums)

(defn prefix-sums [values]
  (loop [i 0 total 0 result [0]]
    (if (< i (count values))
      (let [next-total (+ total (nth values i))]
        (recur (inc i) next-total (conj result next-total)))
      result)))

(defn range-sum [prefix left right]
  (- (nth prefix (inc right)) (nth prefix left)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (let [prefix (prefix-sums [5 1 9 3 7 2 8 6 4 10])]
        (recur (dec n)
               (+ checksum
                  (range-sum prefix 0 4)
                  (range-sum prefix 3 8)
                  (range-sum prefix (mod n 5) (+ 4 (mod n 5))))))
      checksum)))

(defn -main [] (println (benchmark 10000)))
(-main)
