(ns b.logic)
(defn -main [] (println (and true 1 2) (and true nil 2) (or nil false 9) (or nil false)))
(-main)
