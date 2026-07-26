(ns cracking.search.binary)

(defn binary-search [values target]
  (loop [low 0 high (dec (count values))]
    (if (> low high)
      -1
      (let [middle (quot (+ low high) 2)
            value (nth values middle)]
        (cond
          (= value target) middle
          (< value target) (recur (inc middle) high)
          :else (recur low (dec middle)))))))

(defn benchmark [rounds]
  (let [values [2 4 6 8 10 12 14 16 18 20 22 24 26 28 30]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total (binary-search values 22) (binary-search values 7)))
        total))))

(defn -main [] (println (benchmark 50000)))
(-main)
