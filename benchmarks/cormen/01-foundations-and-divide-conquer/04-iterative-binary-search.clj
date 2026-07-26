(ns cormen.foundations.iterative-binary-search)

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
  (let [values [2 5 8 11 14 17 20 23 26 29 32 35 38 41 44 47 50 53 56 59]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (let [present (nth values (mod n (count values)))
              missing (+ 60 (mod n 7))]
          (recur (dec n)
                 (+ checksum
                    (inc (binary-search values present))
                    (inc (binary-search values missing)))))
        checksum))))

(defn -main [] (println (benchmark 40000)))
(-main)
