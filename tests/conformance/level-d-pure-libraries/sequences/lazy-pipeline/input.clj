(ns d.lazy-pipeline)
(defn -main [] (println (take 5 (iterate inc 0))))
(-main)
