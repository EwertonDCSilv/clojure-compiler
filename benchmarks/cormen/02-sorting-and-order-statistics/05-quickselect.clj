(ns cormen.sorting.quickselect)

(defn quickselect [values rank]
  (let [pivot (nth values (quot (count values) 2))
        lower (filter (fn [x] (< x pivot)) values)
        equal (filter (fn [x] (= x pivot)) values)
        higher (filter (fn [x] (> x pivot)) values)]
    (cond
      (< rank (count lower)) (quickselect lower rank)
      (< rank (+ (count lower) (count equal))) pivot
      :else (quickselect higher (- rank (count lower) (count equal))))))

(defn benchmark [rounds]
  (let [values [29 3 41 17 11 37 5 23 31 13 2 43 19 7]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (recur (dec n)
               (+ checksum
                  (quickselect values 3)
                  (quickselect values 7)
                  (quickselect values 11)))
        checksum))))

(defn -main [] (println (benchmark 5000)))
(-main)
