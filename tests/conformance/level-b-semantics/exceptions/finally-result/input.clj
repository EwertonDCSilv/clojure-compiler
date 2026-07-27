(ns b.exceptions.finally-result)
(defn -main []
(println (try 42 (finally (println :finally-normal))))
(println (try (throw 7)
(catch Exception error (+ error 1))
(finally (println :finally-caught)))))
(-main)
