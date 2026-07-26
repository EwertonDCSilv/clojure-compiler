(ns cracking.moderate.peak-population)

;; Cada intervalo é [nascimento morte], com morte inclusiva.
(defn alive-in-year [people year]
  (reduce (fn [total person]
            (+ total
               (if (and (<= (nth person 0) year)
                        (>= (nth person 1) year))
                 1
                 0)))
          0
          people))

(defn peak-population [people first-year last-year]
  (loop [year first-year best-year first-year best-count -1]
    (if (> year last-year)
      (+ (* best-year 10) best-count)
      (let [current (alive-in-year people year)]
        (if (> current best-count)
          (recur (inc year) year current)
          (recur (inc year) best-year best-count))))))

(defn benchmark [rounds]
  (let [people (list [1900 1950] [1920 1980] [1930 1940]
                     [1935 1975] [1945 2000] [1955 1990])]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (peak-population people 1900 2000)))
        total))))

(defn -main [] (println (benchmark 500)))
(-main)
