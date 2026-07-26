(ns cracking.math.prime-count)

(defn prime? [value]
  (if (< value 2)
    false
    (loop [divisor 2]
      (cond
        (> (* divisor divisor) value) true
        (= (mod value divisor) 0) false
        :else (recur (inc divisor))))))

(defn count-primes [limit]
  (loop [n 2 total 0]
    (if (> n limit)
      total
      (recur (inc n) (+ total (if (prime? n) 1 0))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (count-primes 300)))
      total)))

(defn -main [] (println (benchmark 200)))
(-main)
