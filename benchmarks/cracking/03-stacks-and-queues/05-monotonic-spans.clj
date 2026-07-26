(ns cracking.stacks.monotonic-spans)

(defn span-at [values index]
  (let [current (nth values index)]
    (loop [i (dec index) span 1]
      (if (and (>= i 0) (<= (nth values i) current))
        (recur (dec i) (inc span))
        span))))

(defn span-checksum [values]
  (loop [i 0 total 0]
    (if (< i (count values))
      (recur (inc i) (+ total (span-at values i)))
      total)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (span-checksum [100 80 60 70 60 75 85])))
      total)))

(defn -main [] (println (benchmark 3000)))
(-main)
