;; Upstream: Exercism Clojure Track, exercise "knapsack".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/knapsack/.meta/example.clj
;; License: MIT; attribution and local benchmark changes: ../UPSTREAM.md

(ns knapsack)

(defn maximum-value [maximum-weight items]
  (if (empty? items)
    0
    (max
     (if (<= (:weight (first items)) maximum-weight)
       (+
        (:value (first items))
        (maximum-value (- maximum-weight (:weight (first items))) (rest items)))
       0)
     (maximum-value maximum-weight (rest items)))))

(defn benchmark [rounds]
  (let [items [{:weight 2 :value 6}
               {:weight 2 :value 3}
               {:weight 6 :value 5}
               {:weight 5 :value 4}
               {:weight 4 :value 6}
               {:weight 3 :value 5}
               {:weight 7 :value 9}
               {:weight 1 :value 2}]]
    (loop [i 0
           checksum 0]
      (if (= i rounds)
        checksum
        (recur (inc i)
               (+ checksum (maximum-value 15 items)))))))

(defn -main [] (println (benchmark 5000)))

(-main)
