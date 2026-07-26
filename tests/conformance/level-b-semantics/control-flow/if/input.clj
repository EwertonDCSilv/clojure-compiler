(ns b.if)
(defn choose [x] (if x 1 2))
(defn -main [] (println (choose true) (choose false) (choose nil)))
(-main)
