(ns d.builtin-extension)
(defprotocol Summarized (summary [value]))
(extend-type List Summarized
(summary [value] (reduce + 0 value)))
(defn -main []
(println (summary (list)) (summary (list 1 2 3 4))))
(-main)
