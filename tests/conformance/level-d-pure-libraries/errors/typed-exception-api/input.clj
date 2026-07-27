(ns d.typed-exception-api)
(defn classify-failure [value]
(try (throw value)
(catch ArithmeticException error :arithmetic)
(catch Exception error :generic)))
(defn -main [] (println (classify-failure :boom)))
(-main)
