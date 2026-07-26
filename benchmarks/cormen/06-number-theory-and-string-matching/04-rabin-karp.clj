(ns cormen.strings.rabin-karp)

(defn vector-hash [values start length]
  (loop [i 0 hash 0]
    (if (>= i length)
      hash
      (recur (inc i)
             (mod (+ (* hash 31) (nth values (+ start i))) 1000003)))))

(defn highest-power [length]
  (loop [i 1 value 1]
    (if (>= i length)
      value
      (recur (inc i) (mod (* value 31) 1000003)))))

(defn matches-at [text pattern start]
  (loop [i 0]
    (cond
      (>= i (count pattern)) true
      (= (nth text (+ start i)) (nth pattern i)) (recur (inc i))
      :else false)))

(defn rabin-karp [text pattern]
  (let [length (count pattern)
        target (vector-hash pattern 0 length)
        high (highest-power length)]
    (loop [start 0
           current (vector-hash text 0 length)
           matches 0
           checksum 0]
      (if (> (+ start length) (count text))
        (+ (* matches 100) checksum)
        (let [found (and (= current target)
                         (matches-at text pattern start))
              next-start (inc start)]
          (if (> (+ next-start length) (count text))
            (+ (* (if found (inc matches) matches) 100)
               (if found (+ checksum start) checksum))
            (let [without-old
                  (mod (- current (* (nth text start) high)) 1000003)
                  next-hash
                  (mod (+ (* without-old 31)
                          (nth text (+ start length)))
                       1000003)]
              (recur next-start
                     next-hash
                     (if found (inc matches) matches)
                     (if found (+ checksum start) checksum)))))))))

(defn benchmark [rounds]
  (let [text [3 1 4 1 5 9 3 1 4 1 5 9 2 6 3 1 4 1 5 9]
        pattern [3 1 4 1 5 9]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n) (+ checksum (rabin-karp text pattern)))
        checksum))))

(defn -main [] (println (benchmark 10000)))
(-main)
