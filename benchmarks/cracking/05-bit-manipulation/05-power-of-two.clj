(ns cracking.bits.power-of-two)

(defn power-of-two? [value]
  (if (<= value 0)
    false
    (loop [n value]
      (cond
        (= n 1) true
        (odd? n) false
        :else (recur (quot n 2))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total
                (if (power-of-two? 1048576) 1 0)
                (if (power-of-two? 1048575) 0 1)))
      total)))

(defn -main [] (println (benchmark 30000)))
(-main)
