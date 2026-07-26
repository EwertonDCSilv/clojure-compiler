(ns c.concat)
(defn -main [] (println (concat (list 1 2) (list 3 4)) (concat (list) (list 1)) (concat (list :a) (list))))
(-main)
