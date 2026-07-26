(ns cracking.stacks.min-stack)

(defn push-min [stack value]
  (let [current-min (if (empty? stack) value (nth (first stack) 1))]
    (cons [value (if (< value current-min) value current-min)] stack)))

(defn build-min-stack [values]
  (reduce push-min (list) values))

(defn min-checksum [stack]
  (loop [remaining stack total 0]
    (if (empty? remaining)
      total
      (recur (rest remaining) (+ total (nth (first remaining) 1))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (min-checksum (build-min-stack [8 6 7 5 3 0 9 4 2 1]))))
      total)))

(defn -main [] (println (benchmark 1500)))
(-main)
