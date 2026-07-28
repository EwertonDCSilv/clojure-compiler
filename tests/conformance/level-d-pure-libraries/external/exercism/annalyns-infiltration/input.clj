;; Upstream: Exercism Clojure Track, concept exercise "annalyns-infiltration".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/annalyns-infiltration/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns annalyns-infiltration)

(defn can-fast-attack?
  "Returns true if a fast-attack can be made, false otherwise."
  [knight-awake?]
  (not knight-awake?))

(defn can-spy?
  "Returns true if the kidnappers can be spied upon, false otherwise."
  [knight-awake? archer-awake? prisoner-awake?]
  (or knight-awake? archer-awake? prisoner-awake?))

(defn can-signal-prisoner?
  "Returns true if the prisoner can be signalled, false otherwise."
  [archer-awake? prisoner-awake?]
  (and (not archer-awake?)
       prisoner-awake?))

(defn can-free-prisoner?
  "Returns true if prisoner can be freed, false otherwise."
  [knight-awake? archer-awake? prisoner-awake? dog-present?]
  (or (and (not knight-awake?)
           (not archer-awake?)
           prisoner-awake?)
      (and (not archer-awake?)
           dog-present?)))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (can-fast-attack? false))
(println (can-spy? false true false))
(println (can-signal-prisoner? false true))
(println (can-free-prisoner? false false true false))
