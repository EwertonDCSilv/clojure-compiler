;; Compiled portion of clojure.core, preloaded by every `clojure-native build`.
;;
;; This file uses only forms accepted by the native compiler. Functions are
;; intentionally eager unless their docstring states short-circuit behavior.
;; Differences from the JVM implementation are part of the current subset
;; contract rather than promises of future compatibility.

(ns clojure.core)

(defn zero?
  "Returns true when numeric `n` equals zero."
  [n] (= n 0))
(defn pos?
  "Returns true when numeric `n` is greater than zero."
  [n] (> n 0))
(defn neg?
  "Returns true when numeric `n` is less than zero."
  [n] (< n 0))
(defn even?
  "Returns true when fixnum `n` is evenly divisible by two."
  [n] (= (mod n 2) 0))
(defn odd?
  "Returns true when fixnum `n` is not even."
  [n] (not (even? n)))
(defn max
  "Returns the greatest of `x` and the eagerly reduced `more` arguments.

  Requires at least one argument and supports the native numeric comparison
  semantics; it does not implement the JVM numeric tower."
  [x & more] (reduce (fn [a b] (if (> a b) a b)) x more))
(defn min
  "Returns the least of `x` and the eagerly reduced `more` arguments.

  Requires at least one argument and supports the native numeric comparison
  semantics; it does not implement the JVM numeric tower."
  [x & more] (reduce (fn [a b] (if (< a b) a b)) x more))

(defn reduce
  "Eagerly folds `coll` from left to right, starting with `acc`.

  This compiled subset provides only the three-argument arity and does not
  implement `reduced` short-circuiting."
  [f acc coll]
  (if (empty? coll)
    acc
    (recur f (f acc (first coll)) (rest coll))))

(defn map
  "Returns an eagerly realized list containing `(f item)` for each item in `coll`.

  Unlike clojure.core/map on the JVM, this implementation is not lazy, is not
  chunked, and accepts one collection."
  [f coll]
  (if (empty? coll)
    (list)
    (cons (f (first coll)) (map f (rest coll)))))

(defn filter
  "Returns an eagerly realized list of items for which `pred` is truthy.

  Unlike the JVM function, the result is not lazy or chunked."
  [pred coll]
  (cond
    (empty? coll) (list)
    (pred (first coll)) (cons (first coll) (filter pred (rest coll)))
    :else (filter pred (rest coll))))

(defn remove
  "Eagerly returns the items in `coll` for which `pred` is falsey.

  The result is a realized list because this delegates to the compiled `filter`."
  [pred coll]
  (filter (fn [x] (not (pred x))) coll))

(defn reverse
  "Eagerly returns a list containing the items of `coll` in reverse order."
  [coll]
  (reduce (fn [acc x] (cons x acc)) (list) coll))

(defn take
  "Eagerly returns a list of at most the first `n` items of `coll`.

  Non-positive `n` returns an empty list; this implementation is not lazy."
  [n coll]
  (if (if (empty? coll) true (<= n 0))
    (list)
    (cons (first coll) (take (dec n) (rest coll)))))

(defn drop
  "Traverses at most `n` items and returns the remaining collection.

  Non-positive `n` returns `coll` unchanged. Traversal is eager even though the
  returned tail may share structure with the input."
  [n coll]
  (if (if (empty? coll) true (<= n 0))
    coll
    (recur (dec n) (rest coll))))

(defn range
  "Eagerly returns a list of fixnums from zero (inclusive) to `n` (exclusive).

  Only the one-argument finite form is available; negative `n` yields an empty
  list."
  [n]
  (loop [i 0 acc (list)]
    (if (< i n)
      (recur (inc i) (cons i acc))
      (reverse acc))))

;; INVARIANT: the analyzer proves this accumulator linear and non-escaping before
;; rewriting the loop to a structural transient (ADR-0009/ADR-0010).
(defn into
  "Eagerly conjoins every item from `from` into collection `to`.

  Uses transient mutation internally when supported, but returns a persistent
  value and preserves the observable collection semantics."
  [to from]
  (persistent! (reduce (fn [acc x] (conj! acc x)) (transient to) from)))

(defn mapv
  "Eagerly applies `f` to every item in `coll` and returns a persistent vector."
  [f coll]
  (persistent! (reduce (fn [acc x] (conj! acc (f x))) (transient []) coll)))

(defn every?
  "Returns true when `pred` is truthy for every item in `coll`.

  Evaluation stops at the first falsey result."
  [pred coll]
  (cond
    (empty? coll) true
    (pred (first coll)) (recur pred (rest coll))
    :else false))

(defn some
  "Returns the first truthy value produced by `pred`, or nil.

  Evaluation stops at the first truthy result."
  [pred coll]
  (if (empty? coll)
    nil
    (let [r (pred (first coll))]
      (if r r (some pred (rest coll))))))

(defn comp
  "Returns a unary function that applies `g` and then `f`.

  This subset supports exactly two functions and one argument, unlike the
  variadic JVM implementation."
  [f g]
  (fn [x] (f (g x))))

(defn identity
  "Returns `x` unchanged."
  [x] x)

(defn second
  "Returns the second item of `coll`, or nil when it is absent."
  [coll] (first (rest coll)))

(defn last
  "Eagerly traverses `coll` and returns its final item, or nil when empty."
  [coll]
  (if (empty? (rest coll))
    (first coll)
    (recur (rest coll))))

(defn concat
  "Eagerly prepends all items of `a` to sequence `b`.

  This two-collection implementation is not lazy and does not accept the
  variadic JVM arities."
  [a b]
  (reduce (fn [acc x] (cons x acc)) b (reverse a)))

(defn mapcat
  "Eagerly concatenates the collections returned by applying `f` to `coll`.

  The result starts as a list and this subset accepts one input collection."
  [f coll]
  (reduce (fn [acc x] (concat acc (f x))) (list) coll))

(defn count-if
  "Eagerly counts items in `coll` for which `pred` is truthy."
  [pred coll]
  (reduce (fn [acc x] (if (pred x) (inc acc) acc)) 0 coll))

(defn ex-data
  "Returns the payload map carried by a thrown value, or nil for non-map values.

  The native runtime throws data maps directly (there is no JVM `ExceptionInfo`),
  so this is map identity. Native I/O handlers read `:kind` from it."
  [e] (if (map? e) e nil))
