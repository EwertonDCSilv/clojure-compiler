(ns cormen.graphs.bellman-ford)

(defn relax-edges [edges distances]
  (loop [i 0 result distances]
    (if (>= i (count edges))
      result
      (let [edge (nth edges i)
            from (nth edge 0)
            to (nth edge 1)
            weight (nth edge 2)
            source-distance (nth result from)
            candidate (+ source-distance weight)]
        (recur (inc i)
               (if (and (< source-distance 1000000000)
                        (< candidate (nth result to)))
                 (assoc result to candidate)
                 result))))))

(defn shortest-paths [edges vertices source]
  (loop [pass 1
         distances (assoc [1000000000 1000000000 1000000000
                           1000000000 1000000000 1000000000]
                          source
                          0)]
    (if (>= pass vertices)
      distances
      (recur (inc pass) (relax-edges edges distances)))))

(defn distance-sum [distances]
  (reduce + 0 distances))

(defn benchmark [rounds]
  (let [edges [[0 1 6] [0 2 7] [1 2 8] [1 3 5] [1 4 -4]
               [2 3 -3] [2 4 9] [3 1 -2] [4 0 2] [4 3 7] [3 5 4] [5 4 1]]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum (distance-sum (shortest-paths edges 6 0))))
        checksum))))

(defn -main [] (println (benchmark 7000)))
(-main)
