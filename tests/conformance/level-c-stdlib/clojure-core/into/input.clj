(ns c.into)
(defn -main [] (println (into [] (list 1 2)) (into (list) (list 1 2)) (into #{} (list 1 1 2))))
(-main)
