(ns cracking.dp.subset-sum)

(defn false-vector [size]
  (loop [i 0 out []]
    (if (< i size)
      (recur (inc i) (conj out false))
      out)))

(defn add-choice [reachable value target]
  (loop [sum target out reachable]
    (if (< sum value)
      out
      (recur (dec sum)
             (assoc out sum
                    (or (nth out sum)
                        (nth out (- sum value))))))))

(defn subset-sum? [values target]
  (loop [remaining values
         reachable (assoc (false-vector (inc target)) 0 true)]
    (if (empty? remaining)
      (nth reachable target)
      (recur (rest remaining)
             (add-choice reachable (first remaining) target)))))

(defn benchmark [rounds]
  (let [values (list 3 34 4 12 5 2 9 7)]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total
                  (if (subset-sum? values 30) 1 0)
                  (if (subset-sum? values 1) 0 1)))
        total))))

(defn -main [] (println (benchmark 500)))
(-main)
