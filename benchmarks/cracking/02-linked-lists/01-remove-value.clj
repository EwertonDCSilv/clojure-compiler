(ns cracking.lists.remove-value)

(defn remove-value [xs target]
  (loop [remaining xs out (list)]
    (if (empty? remaining)
      (reverse out)
      (let [x (first remaining)]
        (recur (rest remaining)
               (if (= x target) out (cons x out)))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (count (remove-value (list 1 2 3 2 4 2 5 6 2) 2))))
      total)))

(defn -main [] (println (benchmark 3000)))
(-main)
