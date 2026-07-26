(ns b.fixed)
(defn add [a b] (+ a b))
(defn -main [] (println (add 1 2) (add -1 1) (add 20 22)))
(-main)
