(ns b.gc-list)
(defn make [n acc] (if (< n 0) acc (recur (dec n) (cons n acc))))
(defn -main [] (println (count (make 100 (list))) (first (make 100 (list)))))
(-main)
