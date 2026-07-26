(ns b.record)
(defrecord Point [x y])
(defn -main [] (let [p (->Point 3 4)] (println p (:x p) (assoc p :x 9) (= p (->Point 3 4)))))
(-main)
