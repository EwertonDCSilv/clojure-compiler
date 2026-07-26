(ns cormen.strings.naive-matching)

(defn matches-at [text pattern start]
  (loop [i 0]
    (cond
      (>= i (count pattern)) true
      (= (nth text (+ start i)) (nth pattern i)) (recur (inc i))
      :else false)))

(defn match-count [text pattern]
  (loop [start 0 matches 0 checksum 0]
    (if (> (+ start (count pattern)) (count text))
      (+ (* matches 100) checksum)
      (if (matches-at text pattern start)
        (recur (inc start) (inc matches) (+ checksum start))
        (recur (inc start) matches checksum)))))

(defn benchmark [rounds]
  (let [text [1 2 1 2 1 3 1 2 1 2 1 2 4 1 2 1 2 1]
        pattern-a [1 2 1]
        pattern-b [1 2 1 2]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum
                  (match-count text pattern-a)
                  (match-count text pattern-b)))
        checksum))))

(defn -main [] (println (benchmark 12000)))
(-main)
