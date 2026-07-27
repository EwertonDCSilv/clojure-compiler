(ns cormen.structures.chained-hash-table)

(defn hash-index [entry-key size]
  (mod (+ (* entry-key 17) 11) size))

(defn insert-key [table entry-key]
  (let [index (hash-index entry-key (count table))
        bucket (nth table index)]
    (assoc table index (cons entry-key bucket))))

(defn build-table [entry-keys]
  (loop [remaining entry-keys table [(list) (list) (list) (list) (list) (list) (list)]]
    (if (empty? remaining)
      table
      (recur (rest remaining) (insert-key table (first remaining))))))

(defn bucket-contains [bucket target]
  (cond
    (empty? bucket) false
    (= (first bucket) target) true
    :else (recur (rest bucket) target)))

(defn table-contains [table target]
  (bucket-contains
    (nth table (hash-index target (count table)))
    target))

(defn lookup-score [table values]
  (loop [remaining values score 0]
    (if (empty? remaining)
      score
      (recur (rest remaining)
             (+ score
                (if (table-contains table (first remaining))
                  (first remaining)
                  1))))))

(defn benchmark [rounds]
  (let [entry-keys [5 12 19 26 7 14 21 28 3 10 17]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (let [table (build-table entry-keys)]
          (recur (dec n)
                 (+ checksum (lookup-score table [5 19 28 4 11 17]))))
        checksum))))

(defn -main [] (println (benchmark 9000)))
(-main)
