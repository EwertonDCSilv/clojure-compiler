(ns cormen.foundations.horner-polynomial)

(defn horner [coefficients x]
  (loop [i 0 acc 0]
    (if (< i (count coefficients))
      (recur (inc i) (+ (* acc x) (nth coefficients i)))
      acc)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (horner [3 -2 5 0 7 1] 4)
                (horner [1 1 2 3 5 8] (+ 2 (mod n 3)))))
      checksum)))

(defn -main [] (println (benchmark 18000)))
(-main)
