(ns cracking.queues.two-stacks)

(defn normalize-queue [incoming outgoing]
  (if (empty? outgoing)
    [(list) (reverse incoming)]
    [incoming outgoing]))

(defn drain-queue [values]
  (loop [incoming (reverse values) outgoing (list) total 0]
    (let [state (normalize-queue incoming outgoing)
          next-in (nth state 0)
          next-out (nth state 1)]
      (if (empty? next-out)
        total
        (recur next-in (rest next-out) (+ total (first next-out)))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (drain-queue (range 40))))
      total)))

(defn -main [] (println (benchmark 1000)))
(-main)
