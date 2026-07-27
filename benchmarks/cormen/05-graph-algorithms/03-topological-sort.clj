(ns cormen.graphs.topological-sort)

(defn indegrees [graph]
  (loop [node 0 degrees [0 0 0 0 0 0 0 0]]
    (if (>= node (count graph))
      degrees
      (let [neighbors (nth graph node)
            updated
            (loop [i 0 current degrees]
              (if (>= i (count neighbors))
                current
                (let [neighbor (nth neighbors i)]
                  (recur (inc i)
                         (assoc current neighbor
                                (inc (nth current neighbor)))))))]
        (recur (inc node) updated)))))

(defn zero-degree-queue [degrees]
  (loop [i 0 result []]
    (if (>= i (count degrees))
      result
      (recur (inc i)
             (if (= (nth degrees i) 0)
               (conj result i)
               result)))))

(defn release-neighbors [neighbors degrees queue]
  (loop [i 0 current degrees result queue]
    (if (>= i (count neighbors))
      [current result]
      (let [neighbor (nth neighbors i)
            next-degree (dec (nth current neighbor))
            next-degrees (assoc current neighbor next-degree)]
        (recur (inc i)
               next-degrees
               (if (= next-degree 0)
                 (conj result neighbor)
                 result))))))

(defn topological-checksum [graph]
  (loop [head 0
         queue (zero-degree-queue (indegrees graph))
         degrees (indegrees graph)
         position 1
         checksum 0]
    (if (>= head (count queue))
      checksum
      (let [node (nth queue head)
            release-result (release-neighbors (nth graph node) degrees queue)]
        (recur (inc head)
               (nth release-result 1)
               (nth release-result 0)
               (inc position)
               (+ checksum (* position (inc node))))))))

(defn benchmark [rounds]
  (let [graph [[2 3] [3 4] [5] [5 6] [6] [7] [7] []]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n) (+ checksum (topological-checksum graph)))
        checksum))))

(defn -main [] (println (benchmark 8000)))
(-main)
