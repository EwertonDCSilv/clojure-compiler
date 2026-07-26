(ns cracking.moderate.arithmetic-swap)

(defn swap-checksum [left right]
  (let [a (+ left right)
        b (- a right)
        a (- a b)]
    (+ (* a 31) b)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (swap-checksum 12345 67890)))
      total)))

(defn -main [] (println (benchmark 100000)))
(-main)
