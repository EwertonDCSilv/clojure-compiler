(ns cormen.foundations.binary-exponentiation)

(defn fast-power [base exponent]
  (loop [b base e exponent acc 1]
    (if (= e 0)
      acc
      (if (= (mod e 2) 1)
        (recur (* b b) (quot e 2) (* acc b))
        (recur (* b b) (quot e 2) acc)))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (fast-power 3 13)
                (fast-power 5 8)
                (fast-power (+ 2 (mod n 5)) 6)))
      checksum)))

(defn -main [] (println (benchmark 12000)))
(-main)
