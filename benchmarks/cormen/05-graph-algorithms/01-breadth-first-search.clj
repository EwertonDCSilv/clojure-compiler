(ns cormen.graphs.breadth-first-search)

(defn discover-neighbors [neighbors index next-distance distances queue]
  (if (>= index (count neighbors))
    [distances queue]
    (let [neighbor (nth neighbors index)]
      (if (= (nth distances neighbor) -1)
        (recur neighbors
               (inc index)
               next-distance
               (assoc distances neighbor next-distance)
               (conj queue neighbor))
        (recur neighbors (inc index) next-distance distances queue)))))

(defn breadth-first-distances [graph source]
  (loop [head 0
         queue [source]
         distances (assoc [-1 -1 -1 -1 -1 -1 -1 -1] source 0)]
    (if (>= head (count queue))
      distances
      (let [node (nth queue head)
            update (discover-neighbors
                     (nth graph node)
                     0
                     (inc (nth distances node))
                     distances
                     queue)]
        (recur (inc head) (nth update 1) (nth update 0))))))

(defn weighted-sum [values]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (* (inc i) (nth values i))))
      total)))

(defn benchmark [rounds]
  (let [graph [[1 2] [0 3 4] [0 4] [1 5] [1 2 5 6] [3 4 7] [4 7] [5 6]]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum
                  (weighted-sum (breadth-first-distances graph (mod n 3)))))
        checksum))))

(defn -main [] (println (benchmark 9000)))
(-main)
