(ns cormen.dp.matrix-chain-order)

(defn cell [matrix row column]
  (nth (nth matrix row) column))

(defn set-cell [matrix row column value]
  (assoc matrix row (assoc (nth matrix row) column value)))

(defn split-cost [dimensions costs i j]
  (loop [k i best 1000000000]
    (if (>= k j)
      best
      (let [cost (+ (cell costs i k)
                    (cell costs (inc k) j)
                    (* (nth dimensions i)
                       (nth dimensions (inc k))
                       (nth dimensions (inc j))))]
        (recur (inc k) (min best cost))))))

(defn matrix-chain-cost [dimensions]
  (let [amount (dec (count dimensions))
        empty-row [0 0 0 0 0]
        initial [empty-row empty-row empty-row empty-row empty-row]]
    (loop [length 2 costs initial]
      (if (> length amount)
        (cell costs 0 (dec amount))
        (let [updated
              (loop [i 0 current costs]
                (let [j (+ i length -1)]
                  (if (>= j amount)
                    current
                    (recur (inc i)
                           (set-cell current i j
                                     (split-cost dimensions current i j))))))]
          (recur (inc length) updated))))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (matrix-chain-cost [5 10 3 12 5 50])
                (matrix-chain-cost [10 20 5 15 30 8])))
      checksum)))

(defn -main [] (println (benchmark 5000)))
(-main)
