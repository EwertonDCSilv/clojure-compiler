(ns b.capture)
(defn -main [] (let [n 5 f (fn [x] (+ x n))] (println (f 0) (f 5) (f -5))))
(-main)
