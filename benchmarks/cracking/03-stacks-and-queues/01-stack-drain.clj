(ns cracking.stacks.stack-drain)

(defn build-stack [size]
  (loop [i 0 stack (list)]
    (if (< i size)
      (recur (inc i) (cons i stack))
      stack)))

(defn drain-stack [stack]
  (loop [remaining stack checksum 0]
    (if (empty? remaining)
      checksum
      (recur (rest remaining) (+ checksum (first remaining))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (drain-stack (build-stack 64))))
      total)))

(defn -main [] (println (benchmark 1000)))
(-main)
