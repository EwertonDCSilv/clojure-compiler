(ns b.err)
(defn -main [] (+ 1 (recur)))
(-main)
