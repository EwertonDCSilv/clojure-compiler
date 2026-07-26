(ns b.gc-coll)
(defn -main [] (println (map (fn [x] (vector x (inc x))) (range 20))))
(-main)
