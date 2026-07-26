(ns b.hof)
(defn invoke [f x] (f x))
(defn -main [] (println (invoke inc 1) (invoke (fn [x] (* x x)) 4) ((comp inc inc) 5)))
(-main)
