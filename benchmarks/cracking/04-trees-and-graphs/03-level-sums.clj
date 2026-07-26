(ns cracking.trees.level-sums)

(defn level-sum [tree start width]
  (loop [i start end (+ start width) total 0]
    (if (and (< i end) (< i (count tree)))
      (recur (inc i) end (+ total (nth tree i)))
      total)))

(defn all-level-checksum [tree]
  (loop [start 0 width 1 weight 1 total 0]
    (if (< start (count tree))
      (recur (+ start width)
             (* width 2)
             (inc weight)
             (+ total (* weight (level-sum tree start width))))
      total)))

(defn benchmark [rounds]
  (let [tree [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (all-level-checksum tree)))
        total))))

(defn -main [] (println (benchmark 5000)))
(-main)
