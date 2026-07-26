(ns cracking.sorting.bubble)

(defn swap-adjacent [values index]
  (let [left (nth values index)
        right (nth values (inc index))]
    (assoc (assoc values index right) (inc index) left)))

(defn bubble-pass [values limit]
  (loop [i 0 out values]
    (if (< i limit)
      (recur (inc i)
             (if (> (nth out i) (nth out (inc i)))
               (swap-adjacent out i)
               out))
      out)))

(defn bubble-sort [values]
  (loop [limit (dec (count values)) out values]
    (if (> limit 0)
      (recur (dec limit) (bubble-pass out limit))
      out)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (nth (bubble-sort [12 5 9 1 8 2 11 3 10 4 7 6]) 11)))
      total)))

(defn -main [] (println (benchmark 800)))
(-main)
