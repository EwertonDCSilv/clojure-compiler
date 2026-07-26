(ns b.list)
(defn -main [] (println (list) (list 1) (cons 1 (list 2 3)) (first (list 9)) (rest (list 9))))
(-main)
