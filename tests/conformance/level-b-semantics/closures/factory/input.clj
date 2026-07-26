(ns b.factory)
(defn adder [n] (fn [x] (+ n x)))
(defn -main [] (println ((adder 2) 3) ((adder -1) 1) ((adder 20) 22)))
(-main)
