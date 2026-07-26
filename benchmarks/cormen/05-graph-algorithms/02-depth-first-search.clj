(ns cormen.graphs.depth-first-search)

(defn depth-first-visit [graph node visited depth]
  (if (= (nth visited node) 1)
    [visited 0]
    (let [marked (assoc visited node 1)
          neighbors (nth graph node)]
      (loop [i 0 current marked score (* (inc depth) (inc node))]
        (if (>= i (count neighbors))
          [current score]
          (let [visit (depth-first-visit
                        graph
                        (nth neighbors i)
                        current
                        (inc depth))]
            (recur (inc i)
                   (nth visit 0)
                   (+ score (nth visit 1)))))))))

(defn depth-first-score [graph source]
  (nth (depth-first-visit graph source [0 0 0 0 0 0 0 0 0] 0) 1))

(defn benchmark [rounds]
  (let [graph [[1 3] [2 4] [0 5] [4] [5 6] [2 7] [7] [3]]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum
                  (depth-first-score graph (mod n (count graph)))))
        checksum))))

(defn -main [] (println (benchmark 9000)))
(-main)
