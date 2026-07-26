(ns cracking.sorting.insertion)

(defn insert-sorted [value sorted]
  (cond
    (empty? sorted) (list value)
    (<= value (first sorted)) (cons value sorted)
    :else (cons (first sorted) (insert-sorted value (rest sorted)))))

(defn insertion-sort [values]
  (reduce (fn [sorted value] (insert-sorted value sorted))
          (list)
          values))

(defn checksum [values]
  (reduce (fn [total value] (+ (* total 11) value)) 0 values))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (checksum (insertion-sort (list 9 1 8 2 7 3 6 4 5 0)))))
      total)))

(defn -main [] (println (benchmark 1500)))
(-main)
