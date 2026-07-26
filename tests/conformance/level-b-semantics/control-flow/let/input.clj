(ns b.let)
(defn -main [] (println (let [a 20 b 22] (+ a b)) (let [x 1] (let [x 2] x))))
(-main)
