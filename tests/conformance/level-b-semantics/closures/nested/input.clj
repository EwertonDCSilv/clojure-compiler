(ns b.nested)
(defn -main [] (println (((fn [a] (fn [b] (+ a b))) 3) 4)))
(-main)
