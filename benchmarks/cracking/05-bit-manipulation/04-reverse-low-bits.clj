(ns cracking.bits.reverse-low-bits)

(defn reverse-low-bits [value width]
  (loop [n value left width reversed 0]
    (if (= left 0)
      reversed
      (recur (quot n 2)
             (dec left)
             (+ (* reversed 2) (mod n 2))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (reverse-low-bits 178 8)))
      total)))

(defn -main [] (println (benchmark 30000)))
(-main)
