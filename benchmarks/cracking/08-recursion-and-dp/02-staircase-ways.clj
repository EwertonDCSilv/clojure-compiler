(ns cracking.dp.staircase)

(defn staircase-ways [steps]
  (cond
    (= steps 0) 1
    (= steps 1) 1
    (= steps 2) 2
    :else
    (loop [n 3 a 1 b 1 c 2]
      (if (> n steps)
        c
        (recur (inc n) b c (+ a b c))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (staircase-ways 25)))
      total)))

(defn -main [] (println (benchmark 10000)))
(-main)
