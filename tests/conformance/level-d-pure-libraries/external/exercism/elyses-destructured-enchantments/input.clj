;; Upstream: Exercism Clojure Track, concept exercise "elyses-destructured-enchantments".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/elyses-destructured-enchantments/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns elyses-destructured-enchantments)

(defn first-card
  "Returns the first card from deck."
  [[card]]
  card)

(defn second-card
  "Returns the second card from deck."
  [[_ card]]
  card)

(defn swap-top-two-cards
  "Returns the deck with first two items reversed."
  [[a b & more]]
  (list* b a more))

(defn discard-top-card
  "Returns a sequence containing the first card and
   a sequence of the remaining cards in the deck."
  [[card & more]]
  [card more])

(def face-cards
  ["jack" "queen" "king"])

(defn insert-face-cards
  "Returns the deck with face cards between its head and tail."
  [[head & tail]]
    (vec (remove nil? (flatten [head face-cards tail]))))

(comment
  (insert-face-cards [3 10 7])
  (insert-face-cards [9])
  (insert-face-cards []))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (first-card [3 10 7]))
(println (second-card [3 10 7]))
(println (swap-top-two-cards [3 10 7]))
(println (discard-top-card [3 10 7]))
(println (insert-face-cards [3 10 7]))
