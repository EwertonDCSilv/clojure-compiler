(ns b.apply)
(defn add3 [a b c] (+ a b c))
(defn -main [] (println (apply add3 (list 1 2 3)) (apply add3 10 (list 20 30)) (apply add3 [7 8 9])))
(-main)
