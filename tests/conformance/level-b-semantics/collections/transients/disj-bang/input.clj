(ns b.transients.disj)
(defn -main []
(println (persistent! (disj! (transient #{1 2}) 1))))
(-main)
