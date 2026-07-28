;; Upstream: Exercism Clojure Track, concept exercise "lucians-luscious-lasagna".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/lucians-luscious-lasagna/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns lucians-luscious-lasagna)

(def expected-time 40)

(defn remaining-time [actual-time]
  (- expected-time actual-time))

(defn prep-time [num-layers]
  (* num-layers 2))

(defn total-time [num-layers actual-time]
  (+ (prep-time num-layers) actual-time))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (remaining-time 30))
(println (prep-time 3))
(println (total-time 3 20))
