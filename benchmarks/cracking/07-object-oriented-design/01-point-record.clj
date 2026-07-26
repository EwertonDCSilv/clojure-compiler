(ns cracking.objects.point-record)

(defrecord Point [x y])

(defn distance-squared [point]
  (+ (* (:x point) (:x point))
     (* (:y point) (:y point))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total (distance-squared (->Point (mod n 17) (mod n 23)))))
      total)))

(defn -main [] (println (benchmark 10000)))
(-main)
