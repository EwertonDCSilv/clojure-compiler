(ns e.gc-stress-workload)
(defn squares [n] (mapv (fn [x] (* x x)) (range n)))
(defn -main []
(let [values (squares 200)]
(println (count values) (reduce + 0 values))))
(-main)
