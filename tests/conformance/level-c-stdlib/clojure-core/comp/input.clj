(ns c.comp)
(defn -main [] (println ((comp inc inc) 0) ((comp dec inc) 9) ((comp (fn [x] (* x 2)) inc) 4)))
(-main)
