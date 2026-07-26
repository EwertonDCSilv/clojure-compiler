(ns cracking.lists.kth-from-end)

(defn advance [xs steps]
  (loop [remaining xs n steps]
    (if (> n 0)
      (recur (rest remaining) (dec n))
      remaining)))

(defn kth-from-end [xs k]
  (loop [lead (advance xs k) follow xs]
    (if (empty? lead)
      (first follow)
      (recur (rest lead) (rest follow)))))

(defn benchmark [rounds]
  (let [xs (list 10 20 30 40 50 60 70 80 90)]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (kth-from-end xs 4)))
        total))))

(defn -main [] (println (benchmark 10000)))
(-main)
