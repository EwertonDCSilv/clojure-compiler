(ns e.runtime-eval)
(defn -main [] (println (eval '(+ 20 22))))
(-main)
