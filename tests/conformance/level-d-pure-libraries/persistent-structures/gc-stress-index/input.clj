(ns d.gc-stress-index)
(defn build-index [n]
(loop [i 0 index {}]
(if (< i n)
(recur (inc i) (assoc index i [i (* i i)]))
index)))
(defn -main []
(let [index (build-index 80)]
(println (count index) (get index 79) (contains? index 40))))
(-main)
