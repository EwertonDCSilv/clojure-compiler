(ns cracking.lists.stable-partition)

(defn stable-partition [xs pivot]
  (loop [remaining xs lower (list) upper (list)]
    (if (empty? remaining)
      (concat (reverse lower) (reverse upper))
      (let [x (first remaining)]
        (if (< x pivot)
          (recur (rest remaining) (cons x lower) upper)
          (recur (rest remaining) lower (cons x upper)))))))

(defn checksum [xs]
  (reduce (fn [acc x] (+ (* acc 13) x)) 0 xs))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (checksum (stable-partition (list 9 1 8 2 7 3 6 4 5) 5))))
      total)))

(defn -main [] (println (benchmark 1500)))
(-main)
