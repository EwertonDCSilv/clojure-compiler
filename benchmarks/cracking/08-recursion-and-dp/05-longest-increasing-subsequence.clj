(ns cracking.dp.longest-increasing)

(defn one-vector [size]
  (loop [i 0 out []]
    (if (< i size)
      (recur (inc i) (conj out 1))
      out)))

(defn best-before [values lengths index]
  (loop [j 0 best 1]
    (if (< j index)
      (recur (inc j)
             (if (< (nth values j) (nth values index))
               (max best (inc (nth lengths j)))
               best))
      best)))

(defn longest-increasing [values]
  (loop [i 0 lengths (one-vector (count values)) best 0]
    (if (< i (count values))
      (let [current (best-before values lengths i)]
        (recur (inc i) (assoc lengths i current) (max best current)))
      best)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (longest-increasing [10 9 2 5 3 7 101 18 19 20 4 6])))
      total)))

(defn -main [] (println (benchmark 1000)))
(-main)
