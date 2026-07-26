(ns cormen.strings.knuth-morris-pratt)

(defn prefix-table [pattern]
  (loop [i 1 length 0 table [0 0 0 0 0 0 0 0]]
    (if (>= i (count pattern))
      table
      (if (= (nth pattern i) (nth pattern length))
        (let [next-length (inc length)]
          (recur (inc i) next-length (assoc table i next-length)))
        (if (> length 0)
          (recur i (nth table (dec length)) table)
          (recur (inc i) 0 (assoc table i 0)))))))

(defn kmp-count [text pattern]
  (let [prefix (prefix-table pattern)]
    (loop [i 0 j 0 matches 0 checksum 0]
      (if (>= i (count text))
        (+ (* matches 100) checksum)
        (if (= (nth text i) (nth pattern j))
          (if (= (inc j) (count pattern))
            (recur (inc i)
                   (nth prefix j)
                   (inc matches)
                   (+ checksum (- (inc i) (count pattern))))
            (recur (inc i) (inc j) matches checksum))
          (if (> j 0)
            (recur i (nth prefix (dec j)) matches checksum)
            (recur (inc i) 0 matches checksum)))))))

(defn benchmark [rounds]
  (let [text [1 2 1 2 1 2 3 1 2 1 2 3 1 2 1 2 1 2 3]
        pattern [1 2 1 2 3]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n) (+ checksum (kmp-count text pattern)))
        checksum))))

(defn -main [] (println (benchmark 10000)))
(-main)
