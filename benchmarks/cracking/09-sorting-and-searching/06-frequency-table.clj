(ns cracking.search.frequency-table)

(defn frequencies [values]
  (reduce (fn [table value]
            (assoc table value (inc (or (get table value) 0))))
          {}
          values))

(defn checksum [table]
  (reduce (fn [total key]
            (+ total (* key (get table key))))
          0
          (keys table)))

(defn benchmark [rounds]
  (let [values (list 1 2 3 2 1 4 5 4 3 2 1 6 5 4 3 2 1)]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (checksum (frequencies values))))
        total))))

(defn -main [] (println (benchmark 2000)))
(-main)
