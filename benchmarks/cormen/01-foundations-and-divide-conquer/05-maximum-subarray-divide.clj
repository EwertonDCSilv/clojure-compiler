(ns cormen.foundations.maximum-subarray-divide)

(defn best-left-crossing [values low middle]
  (loop [i middle sum 0 best -1000000]
    (if (< i low)
      best
      (let [next-sum (+ sum (nth values i))]
        (recur (dec i) next-sum (max best next-sum))))))

(defn best-right-crossing [values middle high]
  (loop [i (inc middle) sum 0 best -1000000]
    (if (> i high)
      best
      (let [next-sum (+ sum (nth values i))]
        (recur (inc i) next-sum (max best next-sum))))))

(defn maximum-subarray [values low high]
  (if (= low high)
    (nth values low)
    (let [middle (quot (+ low high) 2)
          left (maximum-subarray values low middle)
          right (maximum-subarray values (inc middle) high)
          crossing (+ (best-left-crossing values low middle)
                      (best-right-crossing values middle high))]
      (max left right crossing))))

(defn benchmark [rounds]
  (let [values [13 -3 -25 20 -3 -16 -23 18 20 -7 12 -5 -22 15 -4 7]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum (maximum-subarray values 0 (dec (count values)))))
        checksum))))

(defn -main [] (println (benchmark 8000)))
(-main)
