(ns cormen.dp.rod-cutting)

(defn best-price [prices best length]
  (loop [cut 1 value 0]
    (if (> cut length)
      value
      (recur (inc cut)
             (max value
                  (+ (nth prices cut)
                     (nth best (- length cut))))))))

(defn rod-cut [prices length]
  (loop [current 1 best [0]]
    (if (> current length)
      (nth best length)
      (recur (inc current)
             (conj best (best-price prices best current))))))

(defn benchmark [rounds]
  (let [prices [0 1 5 8 9 10 17 17 20 24 30]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum
                  (rod-cut prices 7)
                  (rod-cut prices 9)
                  (rod-cut prices 10)))
        checksum))))

(defn -main [] (println (benchmark 7000)))
(-main)
