(ns cracking.arrays.compact-adjacent)

(defn compact-adjacent [xs]
  (loop [i 0 previous nil out []]
    (if (< i (count xs))
      (let [x (nth xs i)]
        (if (= x previous)
          (recur (inc i) previous out)
          (recur (inc i) x (conj out x))))
      out)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (count (compact-adjacent [1 1 2 2 2 3 1 1 4 4 5 5 5 6]))))
      total)))

(defn -main [] (println (benchmark 5000)))
(-main)
