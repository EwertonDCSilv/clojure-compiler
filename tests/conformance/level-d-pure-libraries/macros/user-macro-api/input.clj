(ns d.user-macro)
(defmacro unless [predicate value] `(if ~predicate nil ~value))
(defn -main [] (println (unless false 42)))
(-main)
