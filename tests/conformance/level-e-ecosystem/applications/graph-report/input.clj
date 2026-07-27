(ns e.graph-report)
(defn reachable-count [graph start]
(loop [queue (list start) seen #{}]
(if (empty? queue)
(count seen)
(let [node (first queue) remaining (rest queue)]
(if (contains? seen node)
(recur remaining seen)
(recur (concat remaining (get graph node))
(conj seen node)))))))
(defn -main []
(println "reachable"
(reachable-count {0 [1 2] 1 [3] 2 [3 4] 3 [] 4 []} 0)))
(-main)
