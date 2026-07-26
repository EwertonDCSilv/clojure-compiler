(ns cracking.moderate.board-lengths)

(defn board-lengths [boards shorter longer]
  (loop [short-count 0 lengths #{}]
    (if (> short-count boards)
      lengths
      (let [long-count (- boards short-count)
            length (+ (* short-count shorter)
                      (* long-count longer))]
        (recur (inc short-count) (conj lengths length))))))

(defn checksum [lengths]
  (reduce + 0 lengths))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (let [lengths (board-lengths 20 3 7)]
        (recur (dec n) (+ total (count lengths) (checksum lengths))))
      total)))

(defn -main [] (println (benchmark 3000)))
(-main)
