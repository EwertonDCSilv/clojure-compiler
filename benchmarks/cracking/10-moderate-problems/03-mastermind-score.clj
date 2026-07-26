(ns cracking.moderate.mastermind)

(defn exact-matches [secret guess]
  (loop [i 0 total 0]
    (if (< i (count secret))
      (recur (inc i)
             (+ total (if (= (nth secret i) (nth guess i)) 1 0)))
      total)))

(defn color-counts [values]
  (reduce (fn [counts value]
            (assoc counts value (inc (or (get counts value) 0))))
          {}
          values))

(defn total-color-matches [secret guess]
  (let [left (color-counts secret)
        right (color-counts guess)]
    (reduce (fn [total color]
              (+ total (min (get left color)
                            (or (get right color) 0))))
            0
            (keys left))))

(defn score [secret guess]
  (let [exact (exact-matches secret guess)]
    (+ (* exact 10)
       (- (total-color-matches secret guess) exact))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (score [1 2 3 4] [1 3 2 5])))
      total)))

(defn -main [] (println (benchmark 5000)))
(-main)
