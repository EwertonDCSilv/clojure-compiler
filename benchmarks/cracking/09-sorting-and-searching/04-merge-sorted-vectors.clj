(ns cracking.sorting.merge-vectors)

(defn merge-vectors [left right]
  (loop [i 0 j 0 out []]
    (cond
      (>= i (count left))
      (into out (drop j right))
      (>= j (count right))
      (into out (drop i left))
      (<= (nth left i) (nth right j))
      (recur (inc i) j (conj out (nth left i)))
      :else
      (recur i (inc j) (conj out (nth right j))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (let [merged (merge-vectors [1 4 7 10 13 16] [2 3 8 9 14 15])]
        (recur (dec n) (+ total (count merged) (nth merged 11))))
      total)))

(defn -main [] (println (benchmark 2000)))
(-main)
