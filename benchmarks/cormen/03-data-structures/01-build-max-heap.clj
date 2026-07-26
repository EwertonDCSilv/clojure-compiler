(ns cormen.structures.build-max-heap)

(defn swap-at [values left right]
  (let [a (nth values left)
        b (nth values right)]
    (assoc (assoc values left b) right a)))

(defn max-heapify [values start heap-size]
  (loop [index start result values]
    (let [left (+ (* 2 index) 1)
          right (+ left 1)
          largest-left
          (if (and (< left heap-size)
                   (> (nth result left) (nth result index)))
            left
            index)
          largest
          (if (and (< right heap-size)
                   (> (nth result right) (nth result largest-left)))
            right
            largest-left)]
      (if (= largest index)
        result
        (recur largest (swap-at result index largest))))))

(defn build-max-heap [values]
  (loop [i (dec (quot (count values) 2)) result values]
    (if (< i 0)
      result
      (recur (dec i) (max-heapify result i (count result))))))

(defn weighted-sum [values]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (* (inc i) (nth values i))))
      total)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (weighted-sum
                  (build-max-heap [4 1 3 2 16 9 10 14 8 7 11 6 5]))))
      checksum)))

(defn -main [] (println (benchmark 7000)))
(-main)
