(ns d.domain-model)
(defprotocol Costed (cost [item]))
(defrecord LineItem [price quantity])
(extend-type LineItem Costed
(cost [item] (* (:price item) (:quantity item))))
(defn total-cost [items]
(reduce (fn [sum item] (+ sum (cost item))) 0 items))
(defn -main []
(let [items [(->LineItem 10 2) (->LineItem 7 3) (->LineItem 99 0)]]
(println (mapv cost items) (total-cost items))))
(-main)
