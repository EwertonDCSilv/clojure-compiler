(ns cormen.dp.zero-one-knapsack)

(defn apply-item [best weight value capacity]
  (loop [current capacity result best]
    (if (< current weight)
      result
      (recur (dec current)
             (assoc result current
                    (max (nth result current)
                         (+ value (nth result (- current weight)))))))))

(defn knapsack [weights values capacity]
  (loop [item 0
         best [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]]
    (if (>= item (count weights))
      (nth best capacity)
      (recur (inc item)
             (apply-item best
                         (nth weights item)
                         (nth values item)
                         capacity)))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (knapsack [2 3 4 5 9] [3 4 8 8 10] 15)
                (knapsack [1 4 5 7 8] [1 7 9 12 13] 15)))
      checksum)))

(defn -main [] (println (benchmark 7000)))
(-main)
