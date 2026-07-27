(ns e.analytics-cli)
(defn positives [values] (filter pos? values))
(defn -main []
(let [values [-2 0 3 4 -1 5]]
(println "report" (count values) (reduce + 0 values)
(mapv (fn [x] (* x x)) (positives values)))))
(-main)
