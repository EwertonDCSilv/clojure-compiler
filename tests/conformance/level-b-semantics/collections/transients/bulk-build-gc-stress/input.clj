(ns b.transients.bulk)
(defn build-vector [limit]
(loop [index 0 value (transient [])]
(if (< index limit)
(recur (inc index) (conj! value (* index index)))
(persistent! value))))
(defn build-map [limit]
(loop [index 0 value (transient {})]
(if (< index limit)
(recur (inc index) (assoc! value index (inc index)))
(persistent! value))))
(defn -main []
(let [vector-value (build-vector 128)
map-value (build-map 96)]
(println (count vector-value) (nth vector-value 127))
(println (count map-value) (get map-value 95))))
(-main)
