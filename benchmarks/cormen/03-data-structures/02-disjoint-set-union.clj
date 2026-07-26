(ns cormen.structures.disjoint-set-union)

(defn replace-label [labels old-label new-label]
  (loop [i 0 result labels]
    (if (< i (count labels))
      (recur (inc i)
             (if (= (nth labels i) old-label)
               (assoc result i new-label)
               result))
      result)))

(defn union-sets [labels left right]
  (let [left-label (nth labels left)
        right-label (nth labels right)]
    (if (= left-label right-label)
      labels
      (replace-label labels right-label left-label))))

(defn connected-value [labels left right]
  (if (= (nth labels left) (nth labels right)) 1 0))

(defn run-unions []
  (let [a (union-sets [0 1 2 3 4 5 6 7 8 9] 0 1)
        b (union-sets a 2 3)
        c (union-sets b 1 3)
        d (union-sets c 5 6)
        e (union-sets d 6 7)
        f (union-sets e 3 7)]
    (+ (* 10 (connected-value f 0 7))
       (* 20 (connected-value f 2 5))
       (* 30 (connected-value f 0 9))
       (reduce + 0 f))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n) (+ checksum (run-unions)))
      checksum)))

(defn -main [] (println (benchmark 15000)))
(-main)
