(ns cracking.graphs.components)

(defn visit-component [graph start seen]
  (loop [queue (list start) visited seen]
    (if (empty? queue)
      visited
      (let [node (first queue)]
        (if (contains? visited node)
          (recur (rest queue) visited)
          (recur (concat (rest queue) (nth graph node))
                 (conj visited node)))))))

(defn component-count [graph]
  (loop [node 0 seen #{} components 0]
    (if (>= node (count graph))
      components
      (if (contains? seen node)
        (recur (inc node) seen components)
        (recur (inc node)
               (visit-component graph node seen)
               (inc components))))))

(defn benchmark [rounds]
  (let [graph [[1] [0 2] [1] [4] [3] [] [7 8] [6] [6]]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (component-count graph)))
        total))))

(defn -main [] (println (benchmark 2000)))
(-main)
