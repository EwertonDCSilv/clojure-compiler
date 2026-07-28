;; Upstream: Exercism Clojure Track, concept exercise "international-calling-connoisseur".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/international-calling-connoisseur/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns international-calling-connoisseur)

(def countries
  {1 "United States of America", 55 "Brazil", 91 "India"})

(defn add-country [m code name]
  (assoc m code name))

(defn country-name [m code]
  (get m code))

(defn code-exists? [m code]
  (if (get m code) true false))

(defn update-country [m code name]
  (if (code-exists? m code)
    (assoc m code name)
    m))

(defn remove-country [m code]
  (dissoc m code))

(defn longest-name [m]
  (last (first (sort-by count m))))

(comment
  (get countries 1)
  (add-country {} 44 "United Kingdom")
  (code-exists? countries 999)
  (last (first (sort-by count countries)))
  (longest-name {})
  )

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (country-name countries 55))
(println (code-exists? countries 91))
(println (add-country {} 44 "United Kingdom"))
(println (update-country {55 "Brazil"} 55 "Brasil"))
(println (remove-country {1 "USA" 55 "Brazil"} 1))
