(ns cracking.trees.height)

(defn tree-height [tree index]
  (if (>= index (count tree))
    0
    (inc (max (tree-height tree (+ (* index 2) 1))
              (tree-height tree (+ (* index 2) 2))))))

(defn benchmark [rounds]
  (let [tree [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (tree-height tree 0)))
        total))))

(defn -main [] (println (benchmark 5000)))
(-main)
