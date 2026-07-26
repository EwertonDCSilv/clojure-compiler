(ns cormen.greedy.activity-selection)

(defn select-activities [starts finishes]
  (loop [i 0 last-finish -1 chosen 0 finish-sum 0]
    (if (>= i (count starts))
      (+ (* chosen 100) finish-sum)
      (if (>= (nth starts i) last-finish)
        (recur (inc i)
               (nth finishes i)
               (inc chosen)
               (+ finish-sum (nth finishes i)))
        (recur (inc i) last-finish chosen finish-sum)))))

(defn benchmark [rounds]
  (let [starts [1 3 0 5 3 5 6 8 8 2 12]
        finishes [4 5 6 7 9 9 10 11 12 14 16]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum (select-activities starts finishes)))
        checksum))))

(defn -main [] (println (benchmark 18000)))
(-main)
