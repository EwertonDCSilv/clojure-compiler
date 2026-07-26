(ns cracking.lists.merge-sorted)

(defn merge-sorted [left right]
  (cond
    (empty? left) right
    (empty? right) left
    (<= (first left) (first right))
    (cons (first left) (merge-sorted (rest left) right))
    :else
    (cons (first right) (merge-sorted left (rest right)))))

(defn benchmark [rounds]
  (let [left (list 1 3 5 7 9 11)
        right (list 2 4 6 8 10 12)]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (count (merge-sorted left right))))
        total))))

(defn -main [] (println (benchmark 2000)))
(-main)
