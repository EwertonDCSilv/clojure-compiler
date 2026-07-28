;; Upstream: Exercism Clojure Track, concept exercise "interest-is-interesting".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/interest-is-interesting/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns interest-is-interesting)

(defn interest-rate [balance]
  (cond
    (< balance 0.0M) -3.213
    (< balance 1000.0M) 0.5
    (< balance 5000.0M) 1.621
    :else 2.475))

(defn- annual-yield [balance]
  (let [multiplier (bigdec (/ (interest-rate balance)
                              100))]
    (* (abs balance) multiplier)))

(defn annual-balance-update [balance]
  (+ balance (annual-yield balance)))

(defn amount-to-donate [balance tax-free-percentage]
  (if (> balance 0.0M)
    (int (* balance
            (* 2.0 (/ tax-free-percentage 100.0))))
    0))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (interest-rate -1M))
(println (interest-rate 2000M))
(println (annual-balance-update 1000M))
(println (amount-to-donate 550M 2.5))
