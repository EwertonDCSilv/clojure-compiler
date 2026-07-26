(ns cracking.math.trailing-zeros)

(defn trailing-zeros [factorial-of]
  (loop [divisor 5 total 0]
    (if (> divisor factorial-of)
      total
      (recur (* divisor 5) (+ total (quot factorial-of divisor))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (trailing-zeros 1000000) (trailing-zeros 123456)))
      total)))

(defn -main [] (println (benchmark 20000)))
(-main)
