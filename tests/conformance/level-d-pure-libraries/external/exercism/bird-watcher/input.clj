;; Upstream: Exercism Clojure Track, concept exercise "bird-watcher".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/bird-watcher/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns bird-watcher)

(def last-week [0 2 5 3 7 8 4])

(defn today [birds]
  (peek birds))

(defn inc-bird [birds]
  (update birds (dec (count birds)) inc))

(defn day-without-birds? [birds]
  (not (every? pos? birds)))

(defn n-days-count [birds n]
  (reduce + (take n birds)))

(defn busy-days [birds]
  (count (filter #(>= % 5) birds)))

(defn odd-week? [birds]
  (= birds [1 0 1 0 1 0 1]))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (today last-week))
(println (inc-bird last-week))
(println (day-without-birds? last-week))
(println (n-days-count last-week 4))
(println (busy-days last-week))
(println (odd-week? [1 0 1 0 1 0 1]))
