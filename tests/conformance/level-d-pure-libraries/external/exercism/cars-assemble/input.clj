;; Upstream: Exercism Clojure Track, concept exercise "cars-assemble".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/cars-assemble/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns cars-assemble)

(def default-rate 221)

(defn success-rate [speed]
  (cond (= speed 10) 0.77
        (= speed 9) 0.8
        (>= speed 5) 0.9
        :else 1.0))

(defn production-rate
  "Returns the assembly line's production rate per hour,
   taking into account its success rate"
  [speed]
  (* (* default-rate speed)
     (success-rate speed)))

(defn working-items
  "Calculates how many working cars are produced per minute"
  [speed]
  (int (quot (production-rate speed) 60)))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (success-rate 6))
(println (production-rate 6))
(println (working-items 6))
