(ns b.exceptions.try-catch)
(defn -main []
(println (try (throw :boom) (catch Exception error error)))
(println (try 42 (catch Exception error :unexpected))))
(-main)
