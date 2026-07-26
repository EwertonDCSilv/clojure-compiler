(ns cormen.dp.longest-common-subsequence)

(defn cell [table row column]
  (nth (nth table row) column))

(defn set-cell [table row column value]
  (assoc table row (assoc (nth table row) column value)))

(defn lcs-length [left right]
  (let [row [0 0 0 0 0 0 0 0 0 0 0]
        initial [row row row row row row row row row row row]]
    (loop [i 1 table initial]
      (if (> i (count left))
        (cell table (count left) (count right))
        (let [updated
              (loop [j 1 current table]
                (if (> j (count right))
                  current
                  (let [value
                        (if (= (nth left (dec i)) (nth right (dec j)))
                          (inc (cell current (dec i) (dec j)))
                          (max (cell current (dec i) j)
                               (cell current i (dec j))))]
                    (recur (inc j) (set-cell current i j value)))))]
          (recur (inc i) updated))))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (lcs-length [1 2 3 2 4 1 2] [2 4 3 1 2 1])
                (lcs-length [7 1 5 9 2 6 3 8] [1 9 5 2 3 6 8])))
      checksum)))

(defn -main [] (println (benchmark 4500)))
(-main)
