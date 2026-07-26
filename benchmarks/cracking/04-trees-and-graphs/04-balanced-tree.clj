(ns cracking.trees.balanced)

(defn abs-int [n] (if (< n 0) (- 0 n) n))

(defn subtree-height [tree index]
  (if (or (>= index (count tree)) (= (nth tree index) -1))
    0
    (inc (max (subtree-height tree (+ (* index 2) 1))
              (subtree-height tree (+ (* index 2) 2))))))

(defn balanced-root? [tree]
  (<= (abs-int (- (subtree-height tree 1)
                  (subtree-height tree 2)))
      1))

(defn benchmark [rounds]
  (let [balanced [1 2 3 4 5 6 7]
        skewed [1 2 -1 3 -1 -1 -1 4]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total
                  (if (balanced-root? balanced) 1 0)
                  (if (balanced-root? skewed) 0 1)))
        total))))

(defn -main [] (println (benchmark 4000)))
(-main)
