(ns cracking.bits.insert-field)

(defn pow-two [exponent]
  (loop [n exponent value 1]
    (if (> n 0)
      (recur (dec n) (* value 2))
      value)))

(defn insert-field [target field start width]
  (let [low-base (pow-two start)
        field-base (pow-two width)
        high-base (* low-base field-base)
        low (mod target low-base)
        high (* (quot target high-base) high-base)
        inserted (* (mod field field-base) low-base)]
    (+ high inserted low)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (insert-field 1024 19 2 5)))
      total)))

(defn -main [] (println (benchmark 20000)))
(-main)
