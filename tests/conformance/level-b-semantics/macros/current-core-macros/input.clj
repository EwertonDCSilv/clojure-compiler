(ns b.macros)
(defn -main [] (when (and true true) (println (cond false 1 :else (or nil 42)))))
(-main)
