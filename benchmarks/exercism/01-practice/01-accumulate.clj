;; Derived from exercism/clojure, exercise "accumulate", reference solution.
;; Upstream commit: 4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190
;; License: MIT, see ../LICENSE.exercism.

(ns accumulate)

(defn accumulate [f xs]
  (loop [xs xs
         accum []]
    (if
     (empty? xs) accum
     (recur (rest xs) (conj accum (f (first xs)))))))

(defn benchmark [rounds]
  (loop [i 0
         checksum 0]
    (if (= i rounds)
      checksum
      (let [values (accumulate (fn [x] (+ x i)) [1 2 3 4 5 6 7 8])]
        (recur (inc i) (reduce + checksum values))))))

(defn -main [] (println (benchmark 40000)))

(-main)
