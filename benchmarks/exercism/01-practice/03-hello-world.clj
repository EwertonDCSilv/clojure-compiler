;; Upstream: Exercism Clojure Track, exercise "hello-world".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/hello-world/.meta/example.clj
;; License: MIT; attribution and local benchmark changes: ../UPSTREAM.md

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
