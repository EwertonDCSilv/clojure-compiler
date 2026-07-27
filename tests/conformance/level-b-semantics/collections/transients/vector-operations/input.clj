(ns b.transients.vector)
(defn -main []
(let [value (transient [10 20 30])
value (assoc! value 1 99)
value (conj! value 40)]
(println (count value) (nth value 1) (get value 3))
(println (persistent! value)))
(println (persistent! (transient []))))
(-main)
