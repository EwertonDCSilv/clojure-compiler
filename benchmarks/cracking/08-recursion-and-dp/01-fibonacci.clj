(ns cracking.dp.fibonacci)

(defn fibonacci [value]
  (loop [n value previous 0 current 1]
    (if (= n 0)
      previous
      (recur (dec n) current (+ previous current)))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (fibonacci 35)))
      total)))

(defn -main [] (println (benchmark 20000)))
(-main)
