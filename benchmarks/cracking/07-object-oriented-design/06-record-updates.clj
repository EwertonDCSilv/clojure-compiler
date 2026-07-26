(ns cracking.objects.record-updates)

(defrecord Counter [value updates])

(defn increment-counter [counter]
  (assoc (assoc counter :value (inc (:value counter)))
         :updates
         (inc (:updates counter))))

(defn run-counter [steps]
  (loop [n steps counter (->Counter 0 0)]
    (if (> n 0)
      (recur (dec n) (increment-counter counter))
      (+ (:value counter) (:updates counter)))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (run-counter 40)))
      total)))

(defn -main [] (println (benchmark 1500)))
(-main)
