;; Derived from exercism/clojure, exercise "prime-factors", reference solution.
;; Upstream commit: 4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190
;; License: MIT, see ../LICENSE.exercism.

(ns prime-factors)

(defn of
  [n]
  (loop [result []
         candidate 2
         n n]
    (if (= n 1)
      result
      (if (zero? (mod n candidate))
        (recur (conj result candidate) candidate (quot n candidate))
        (recur result (inc candidate) n)))))

(defn benchmark [rounds]
  (loop [i 0
         checksum 0]
    (if (= i rounds)
      checksum
      (recur (inc i)
             (+ checksum (count (of (+ 10000 (mod i 997)))))))))

(defn -main [] (println (benchmark 15000)))

(-main)
