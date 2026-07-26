(ns b.var)
(defn collect [x & xs] (cons x xs))
(defn -main [] (println (collect 1) (collect 1 2) (collect 1 2 3 4)))
(-main)
