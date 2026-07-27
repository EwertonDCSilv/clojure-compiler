(ns e.future-api)
(defn -main [] (println @(future (+ 20 22))))
(-main)
