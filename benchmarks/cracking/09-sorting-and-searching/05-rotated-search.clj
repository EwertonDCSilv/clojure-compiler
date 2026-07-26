(ns cracking.search.rotated)

(defn rotated-search [values target]
  (loop [low 0 high (dec (count values))]
    (if (> low high)
      -1
      (let [middle (quot (+ low high) 2)
            low-value (nth values low)
            middle-value (nth values middle)
            high-value (nth values high)]
        (cond
          (= middle-value target) middle
          (<= low-value middle-value)
          (if (and (>= target low-value) (< target middle-value))
            (recur low (dec middle))
            (recur (inc middle) high))
          :else
          (if (and (> target middle-value) (<= target high-value))
            (recur (inc middle) high)
            (recur low (dec middle))))))))

(defn benchmark [rounds]
  (let [values [15 18 21 24 27 1 3 6 9 12]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total (rotated-search values 6) (rotated-search values 20)))
        total))))

(defn -main [] (println (benchmark 40000)))
(-main)
