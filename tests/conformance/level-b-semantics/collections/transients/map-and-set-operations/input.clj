(ns b.transients.map-set)
(defn -main []
(let [value (transient {:a 1 :b 2})
value (assoc! value :c 3)
value (dissoc! value :b)]
(println (count value) (get value :c) (contains? value :b))
(println (persistent! value)))
(let [value (conj! (conj! (transient #{1}) 2) 3)]
(println (count value) (contains? value 2) (persistent! value))))
(-main)
