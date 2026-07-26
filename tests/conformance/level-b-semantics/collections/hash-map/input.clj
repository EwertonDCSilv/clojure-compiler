(ns b.hmap)
(defn make [n] (loop [i 0 m {}] (if (< i n) (recur (inc i) (assoc m i (* i i))) m)))
(defn -main [] (let [m (make 100)] (println (count m) (get m 99) (contains? m 50) (count (dissoc m 50)))))
(-main)
