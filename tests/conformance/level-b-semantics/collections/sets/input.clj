(ns b.set)
(defn -main [] (println #{1 2 1} (count #{}) (contains? #{1 2} 2) (conj #{1 2} 3)))
(-main)
