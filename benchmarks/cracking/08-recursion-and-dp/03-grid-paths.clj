(ns cracking.dp.grid-paths)

(defn one-vector [size]
  (loop [i 0 out []]
    (if (< i size)
      (recur (inc i) (conj out 1))
      out)))

(defn update-row [row]
  (loop [column 1 out row]
    (if (< column (count out))
      (recur (inc column)
             (assoc out column (+ (nth out (dec column))
                                  (nth out column))))
      out)))

(defn grid-paths [rows columns]
  (loop [row 1 values (one-vector columns)]
    (if (< row rows)
      (recur (inc row) (update-row values))
      (nth values (dec columns)))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (grid-paths 10 10)))
      total)))

(defn -main [] (println (benchmark 1000)))
(-main)
