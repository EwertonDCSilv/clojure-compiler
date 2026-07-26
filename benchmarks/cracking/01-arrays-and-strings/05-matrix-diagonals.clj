(ns cracking.arrays.matrix-diagonals)

(defn diagonal-sum [matrix size]
  (loop [i 0 total 0]
    (if (< i size)
      (let [left (nth matrix (+ (* i size) i))
            right (nth matrix (+ (* i size) (- (dec size) i)))]
        (recur (inc i) (+ total left right)))
      total)))

(defn benchmark [rounds]
  (let [matrix [1 2 3 4 5 6
                7 8 9 10 11 12
                13 14 15 16 17 18
                19 20 21 22 23 24
                25 26 27 28 29 30
                31 32 33 34 35 36]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (diagonal-sum matrix 6)))
        total))))

(defn -main [] (println (benchmark 4000)))
(-main)
