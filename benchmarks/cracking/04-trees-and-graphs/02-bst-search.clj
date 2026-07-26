(ns cracking.trees.bst-search)

(defn bst-search [tree target]
  (loop [index 0 steps 0]
    (if (>= index (count tree))
      (- 0 steps)
      (let [value (nth tree index)]
        (cond
          (= value target) (inc steps)
          (< target value) (recur (+ (* index 2) 1) (inc steps))
          :else (recur (+ (* index 2) 2) (inc steps)))))))

(defn benchmark [rounds]
  (let [tree [16 8 24 4 12 20 28 2 6 10 14 18 22 26 30]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total (bst-search tree 22) (bst-search tree 7)))
        total))))

(defn -main [] (println (benchmark 10000)))
(-main)
