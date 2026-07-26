(ns cormen.number.sieve-of-eratosthenes)

(defn initial-flags [limit]
  (loop [i 0 result []]
    (if (> i limit)
      (assoc (assoc result 0 false) 1 false)
      (recur (inc i) (conj result true)))))

(defn mark-multiples [flags prime limit]
  (loop [multiple (* prime prime) result flags]
    (if (> multiple limit)
      result
      (recur (+ multiple prime) (assoc result multiple false)))))

(defn count-primes [limit]
  (loop [prime 2 flags (initial-flags limit)]
    (if (> (* prime prime) limit)
      (loop [i 2 total 0]
        (if (> i limit)
          total
          (recur (inc i) (if (nth flags i) (inc total) total))))
      (recur (inc prime)
             (if (nth flags prime)
               (mark-multiples flags prime limit)
               flags)))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (count-primes 100)
                (count-primes 150)
                (count-primes (+ 80 (mod n 20)))))
      checksum)))

(defn -main [] (println (benchmark 3500)))
(-main)
