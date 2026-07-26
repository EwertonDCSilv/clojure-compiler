(ns cracking.lists.reversed-digits)

(defn digits-to-number [digits]
  (loop [remaining (reverse digits) value 0]
    (if (empty? remaining)
      value
      (recur (rest remaining) (+ (* value 10) (first remaining))))))

(defn add-reversed-digits [left right]
  (+ (digits-to-number left) (digits-to-number right)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (add-reversed-digits (list 7 1 6) (list 5 9 2))))
      total)))

(defn -main [] (println (benchmark 10000)))
(-main)
