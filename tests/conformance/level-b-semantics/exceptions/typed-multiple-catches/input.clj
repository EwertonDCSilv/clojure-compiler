(ns b.exceptions.typed-catches)
(defn -main []
(println
(try (throw :boom)
(catch ArithmeticException error :arithmetic)
(catch Exception error :generic))))
(-main)
