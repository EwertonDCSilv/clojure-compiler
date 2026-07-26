(ns b.map)
(defn -main [] (let [m {:a 1 :b 2}] (println (get m :a) (:b m) (contains? m :c) (assoc m :c 3) (dissoc m :a))))
(-main)
