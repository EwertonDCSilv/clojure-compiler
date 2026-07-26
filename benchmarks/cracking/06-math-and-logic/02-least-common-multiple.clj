(ns cracking.math.lcm)

(defn gcd [left right]
  (loop [a left b right]
    (if (= b 0) a (recur b (mod a b)))))

(defn lcm [left right]
  (* (quot left (gcd left right)) right))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (lcm 84 120) (lcm 72 90)))
      total)))

(defn -main [] (println (benchmark 50000)))
(-main)
