(ns cracking.graphs.reachability)

(defn reachable? [graph source target]
  (loop [queue (list source) seen #{}]
    (if (empty? queue)
      false
      (let [node (first queue)]
        (cond
          (= node target) true
          (contains? seen node) (recur (rest queue) seen)
          :else (recur (concat (rest queue) (nth graph node))
                       (conj seen node)))))))

(defn benchmark [rounds]
  (let [graph [[1 2] [3] [3 4] [5] [5] [] [0]]]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total
                  (if (reachable? graph 0 5) 1 0)
                  (if (reachable? graph 5 0) 0 1)))
        total))))

(defn -main [] (println (benchmark 3000)))
(-main)
