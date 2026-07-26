(ns cracking.math.modular-power)

(defn modular-power [base exponent modulus]
  (loop [b (mod base modulus) e exponent result 1]
    (if (= e 0)
      result
      (recur (mod (* b b) modulus)
             (quot e 2)
             (if (odd? e) (mod (* result b) modulus) result)))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (modular-power 17 12345 1000003)))
      total)))

(defn -main [] (println (benchmark 20000)))
(-main)
