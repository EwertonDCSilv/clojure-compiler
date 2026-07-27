(ns b.exceptions.nested-unwind)
(defn -main []
(let [prefix "caught:"]
(println
(try
(try (throw (str prefix "value"))
(finally (println :inner-finally)))
(catch Exception error (str error "!"))
(finally (println :outer-finally))))))
(-main)
