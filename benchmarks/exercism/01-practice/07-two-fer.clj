;; Derived from exercism/clojure, exercise "two-fer", reference solution.
;; Upstream commit: 4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190
;; License: MIT, see ../LICENSE.exercism.

(ns two-fer)

(defn two-fer
  ([] (str "One for you, one for me."))
  ([name] (str "One for " name ", one for me.")))

(defn benchmark [rounds]
  (loop [i 0
         checksum 0]
    (if (= i rounds)
      checksum
      (recur (inc i)
             (if (= (two-fer "Alice") "One for Alice, one for me.")
               (inc checksum)
               checksum)))))

(defn -main [] (println (benchmark 300000)))

(-main)
