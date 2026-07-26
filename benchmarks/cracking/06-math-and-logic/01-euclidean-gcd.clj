(ns cracking.math.gcd)

(defn gcd [left right]
  (loop [a left b right]
    (if (= b 0)
      a
      (recur b (mod a b)))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (gcd 1234560 78960) (gcd 987654 123456)))
      total)))

(defn -main [] (println (benchmark 100000)))
(-main)
