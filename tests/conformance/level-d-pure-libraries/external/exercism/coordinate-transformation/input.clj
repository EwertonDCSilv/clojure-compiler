;; Upstream: Exercism Clojure Track, concept exercise "coordinate-transformation".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/coordinate-transformation/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns coordinate-transformation)

(defn translate2d
  "Returns a function making use of a closure to
   perform a repeatable 2d translation of a coordinate pair."
  [dx dy]
  (fn [x y] [(+ dx x) (+ dy y)]))

(defn scale2d
  "Returns a function making use of a closure to
   perform a repeatable 2d scale of a coordinate pair."
  [sx sy]
  (fn [x y] [(* sx x) (* sy y)]))

(defn compose-transform
  "Create a composition function that returns a function that
   combines two functions to perform a repeatable transformation."
  [f g]
  (fn [x y]
    (let [f-result (f x y)]
      (g (first f-result) (last f-result)))))

(defn memoize-transform
  "Returns a function that memoizes the last result.
   If the arguments are the same as the last call,
   the memoized result is returned."
  [f]
  (let [mem (atom {:last-x nil
                   :last-y nil
                   :last-result nil})]
    (fn [& args]
      (if-let [e (and
                  (= (first args) (:last-x @mem))
                  (= (last args) (:last-y @mem))
                  (:last-result @mem))]
        e
        (let [ret (apply f args)]
          (swap! mem assoc :last-x (first args))
          (swap! mem assoc :last-y (last args))
          (swap! mem assoc :last-result ret)
          ret)))))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println ((translate2d 2 3) 4 5))
(println ((scale2d 2 3) 4 5))
(println ((compose-transform (translate2d 2 3) (scale2d 2 3)) 4 5))
(let [transform (memoize-transform (translate2d 2 3))]
  (println (transform 4 5))
  (println (transform 4 5)))
