(ns cracking.dp.coin-change)

(defn zero-vector [size]
  (loop [i 0 out []]
    (if (< i size)
      (recur (inc i) (conj out 0))
      out)))

(defn apply-coin [ways coin amount]
  (loop [value coin out ways]
    (if (> value amount)
      out
      (recur (inc value)
             (assoc out value
                    (+ (nth out value)
                       (nth out (- value coin))))))))

(defn coin-change [amount coins]
  (loop [remaining coins ways (assoc (zero-vector (inc amount)) 0 1)]
    (if (empty? remaining)
      (nth ways amount)
      (recur (rest remaining)
             (apply-coin ways (first remaining) amount)))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (coin-change 50 (list 1 2 5 10 20))))
      total)))

(defn -main [] (println (benchmark 400)))
(-main)
