(ns e.invoice-cli)
(defrecord Item [price quantity])
(defn line-total [item] (* (:price item) (:quantity item)))
(defn invoice-total [items]
(reduce (fn [sum item] (+ sum (line-total item))) 0 items))
(defn -main []
(let [items [(->Item 12 2) (->Item 5 3) (->Item 9 1)]]
(println "invoice" (count items) (invoice-total items))))
(-main)
