(ns cracking.moderate.maximum-subarray)

(defn maximum-subarray [values]
  (loop [i 1
         current (nth values 0)
         best (nth values 0)]
    (if (< i (count values))
      (let [value (nth values i)
            next-current (max value (+ current value))]
        (recur (inc i) next-current (max best next-current)))
      best)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (maximum-subarray [-2 1 -3 4 -1 2 1 -5 4 3 -2 3])))
      total)))

(defn -main [] (println (benchmark 10000)))
(-main)
