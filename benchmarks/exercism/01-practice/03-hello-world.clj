;; Derived from exercism/clojure, exercise "hello-world", reference solution.
;; Upstream commit: 4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190
;; License: MIT, see ../LICENSE.exercism.

(ns hello-world)

(defn hello []
  "Hello, World!")

(defn benchmark [rounds]
  (loop [i 0
         checksum 0]
    (if (= i rounds)
      checksum
      (recur (inc i)
             (if (= (hello) "Hello, World!")
               (inc checksum)
               checksum)))))

(defn -main [] (println (benchmark 500000)))

(-main)
