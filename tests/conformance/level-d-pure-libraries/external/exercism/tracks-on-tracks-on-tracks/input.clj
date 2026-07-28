;; Upstream: Exercism Clojure Track, concept exercise "tracks-on-tracks-on-tracks".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/tracks-on-tracks-on-tracks/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns tracks-on-tracks-on-tracks)

(defn new-list []
  '())

(defn add-language
  [lang-list lang]
  (conj lang-list lang))

(defn first-language
  [lang-list]
  (first lang-list))

(defn remove-language
  [lang-list]
  (rest lang-list))

(defn count-languages
  [lang-list]
  (count lang-list))

(defn learning-list []
  (-> (new-list)
      (add-language "Clojure")
      (add-language "Lisp")
      remove-language
      (add-language "Java")
      (add-language "JavaScript")
      count-languages))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (new-list))
(println (add-language (new-list) "Clojure"))
(println (first-language '("Clojure" "Lisp")))
(println (remove-language '("Clojure" "Lisp")))
(println (learning-list))
