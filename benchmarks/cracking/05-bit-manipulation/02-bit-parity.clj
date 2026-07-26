(ns cracking.bits.parity)

(defn bit-parity [value]
  (loop [n value parity 0]
    (if (= n 0)
      parity
      (recur (quot n 2) (mod (+ parity (mod n 2)) 2)))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (bit-parity 43690) (bit-parity 65535) (bit-parity n)))
      total)))

(defn -main [] (println (benchmark 30000)))
(-main)
