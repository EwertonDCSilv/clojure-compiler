(ns b.user-macro)
(defmacro twice [x] `(+ ~x ~x))
(defn -main [] (println (twice 21)))
(-main)
