(ns b.multi)
(defn greet ([] "hi") ([x] (str "hi " x)) ([a b] (str a " " b)))
(defn -main [] (println (greet) (greet "x") (greet "a" "b")))
(-main)
