(ns cracking.bits.popcount)

;; Implementação aritmética equivalente para exercitar o compilador enquanto
;; primitivas bitwise ainda não fazem parte do subconjunto.
(defn popcount [value]
  (loop [n value bits 0]
    (if (= n 0)
      bits
      (recur (quot n 2) (+ bits (mod n 2))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (popcount 1048575) (popcount 1234567)))
      total)))

(defn -main [] (println (benchmark 20000)))
(-main)
