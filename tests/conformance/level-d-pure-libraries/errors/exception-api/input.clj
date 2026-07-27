(ns d.exception-api)
(defn require-positive [value]
(if (pos? value) value (throw :not-positive)))
(defn -main []
(println (require-positive 7))
(println (try (require-positive 0)
(catch Exception error [:caught error])
(finally (println :validated)))))
(-main)
