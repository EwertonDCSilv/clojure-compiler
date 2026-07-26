(ns cracking.math.integer-square-root)

(defn integer-square-root [value]
  (loop [low 0 high value answer 0]
    (if (> low high)
      answer
      (let [middle (quot (+ low high) 2)
            square (* middle middle)]
        (cond
          (= square value) middle
          (< square value) (recur (inc middle) high middle)
          :else (recur low (dec middle) answer))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (integer-square-root 999999) (integer-square-root 1048576)))
      total)))

(defn -main [] (println (benchmark 20000)))
(-main)
