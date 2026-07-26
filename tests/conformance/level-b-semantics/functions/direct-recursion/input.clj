(ns b.rec)
(defn fib [n] (if (< n 2) n (+ (fib (dec n)) (fib (- n 2)))))
(defn -main [] (println (fib 0) (fib 1) (fib 10)))
(-main)
