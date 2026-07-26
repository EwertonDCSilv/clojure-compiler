(ns cracking.moderate.pair-sum)

(defn pairs-from [values start target]
  (loop [j (inc start) total 0]
    (if (< j (count values))
      (recur (inc j)
             (+ total
                (if (= (+ (nth values start) (nth values j)) target)
                  1
                  0)))
      total)))

(defn pair-sum-count [values target]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (pairs-from values i target)))
      total)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (pair-sum-count [1 5 7 -1 5 3 9 2 6 4] 8)))
      total)))

(defn -main [] (println (benchmark 5000)))
(-main)
