(ns b.vec)
(defn -main [] (println [1 2 3] (nth [10 20 30] 2) (assoc [1 2 3] 1 9) (conj [] 1)))
(-main)
