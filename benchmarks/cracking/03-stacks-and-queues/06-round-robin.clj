(ns cracking.queues.round-robin)

(defn rotate-queue [queue]
  (concat (rest queue) (list (first queue))))

(defn schedule [queue turns]
  (loop [remaining queue n turns total 0]
    (if (> n 0)
      (recur (rotate-queue remaining) (dec n) (+ total (first remaining)))
      total)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (schedule (list 3 5 7 11 13) 50)))
      total)))

(defn -main [] (println (benchmark 500)))
(-main)
