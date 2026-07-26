(ns cormen.graphs.floyd-warshall)

(defn cell [matrix row column]
  (nth (nth matrix row) column))

(defn set-cell [matrix row column value]
  (assoc matrix row (assoc (nth matrix row) column value)))

(defn floyd-warshall [matrix]
  (let [size (count matrix)]
    (loop [k 0 distances matrix]
      (if (>= k size)
        distances
        (let [updated
              (loop [i 0 current distances]
                (if (>= i size)
                  current
                  (let [row-updated
                        (loop [j 0 row-current current]
                          (if (>= j size)
                            row-current
                            (let [left (cell row-current i k)
                                  right (cell row-current k j)
                                  through (+ left right)]
                              (recur (inc j)
                                     (if (and (< left 999999)
                                              (< right 999999)
                                              (< through (cell row-current i j)))
                                       (set-cell row-current i j through)
                                       row-current)))))]
                    (recur (inc i) row-updated))))]
          (recur (inc k) updated))))))

(defn matrix-sum [matrix]
  (loop [i 0 total 0]
    (if (>= i (count matrix))
      total
      (recur (inc i) (+ total (reduce + 0 (nth matrix i)))))))

(defn benchmark [rounds]
  (let [matrix [[0 3 8 999999 -4]
                [999999 0 999999 1 7]
                [999999 4 0 999999 999999]
                [2 999999 -5 0 999999]
                [999999 999999 999999 6 0]]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum (matrix-sum (floyd-warshall matrix))))
        checksum))))

(defn -main [] (println (benchmark 2500)))
(-main)
