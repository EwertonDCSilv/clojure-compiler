;; Derived from exercism/clojure, exercise "square-root", reference solution.
;; Upstream commit: 4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190
;; License: MIT, see ../LICENSE.exercism.

(ns square-root)

(defn square-root
  "Calculates a number's square root"
  [n]
  (loop [i 1]
    (if (<= (* i i) n)
      (recur (inc i))
      (dec i))))

(defn benchmark [rounds]
  (loop [i 0
         checksum 0]
    (if (= i rounds)
      checksum
      (recur (inc i)
             (+ checksum (square-root (+ 10000 (mod i 997))))))))

(defn -main [] (println (benchmark 20000)))

(-main)
