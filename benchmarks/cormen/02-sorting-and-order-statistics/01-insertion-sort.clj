(ns cormen.sorting.insertion-sort)

(defn insertion-sort [values]
  (loop [i 1 result values]
    (if (>= i (count result))
      result
      (let [key (nth result i)
            shifted
            (loop [j (dec i) current result]
              (if (and (>= j 0) (> (nth current j) key))
                (recur (dec j) (assoc current (inc j) (nth current j)))
                (assoc current (inc j) key)))]
        (recur (inc i) shifted)))))

(defn weighted-sum [values]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (* (inc i) (nth values i))))
      total)))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (weighted-sum
                  (insertion-sort [31 4 18 9 27 1 16 7 25 2 20 11]))))
      checksum)))

(defn -main [] (println (benchmark 6000)))
(-main)
