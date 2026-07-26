(ns b.thread)
(defn -main [] (println (-> 10 inc inc) (-> 5 (* 2) (+ 1))))
(-main)
