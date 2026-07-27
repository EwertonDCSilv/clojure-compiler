(ns d.exception-api)
(defn -main []
(println (try (quot 1 0) (catch Exception error :caught))))
(-main)
