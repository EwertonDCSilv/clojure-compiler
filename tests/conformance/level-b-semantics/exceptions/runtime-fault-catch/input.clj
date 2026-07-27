(ns b.exceptions.runtime-fault)
(defn -main []
(println (try (quot 1 0) (catch Exception error :caught))))
(-main)
