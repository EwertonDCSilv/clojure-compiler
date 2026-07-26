(ns b.err)
(defn -main [] (loop [a 1] (recur a a)))
(-main)
