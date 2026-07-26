;; core.clj — parte de clojure.core escrita no próprio dialeto (bootstrap).
;; Ver specs/STANDARD_LIBRARY_SCOPE.md (categoria C) e ADR-0005.
;; Estas definições usam apenas primitivas em Rust + formas especiais.
;; Versões eager/recursivas (lazy-seq é [FUTURO], Fase 6+).

(ns clojure.core)

(defn identity [x] x)

(defn reduce
  ([f coll]
   (if (empty? coll)
     (f)
     (reduce f (first coll) (rest coll))))
  ([f acc coll]
   (if (empty? coll)
     acc
     (recur f (f acc (first coll)) (rest coll)))))

(defn reverse [coll]
  (reduce (fn [acc x] (cons x acc)) (list) coll))

(defn map [f coll]
  (if (empty? coll)
    (list)
    (cons (f (first coll)) (map f (rest coll)))))

(defn mapv [f coll]
  (reduce (fn [acc x] (conj acc (f x))) [] coll))

(defn filter [pred coll]
  (cond
    (empty? coll) (list)
    (pred (first coll)) (cons (first coll) (filter pred (rest coll)))
    :else (filter pred (rest coll))))

(defn remove [pred coll]
  (filter (fn [x] (not (pred x))) coll))

(defn take [n coll]
  (if (if (empty? coll) true (<= n 0))
    (list)
    (cons (first coll) (take (- n 1) (rest coll)))))

(defn drop [n coll]
  (if (if (empty? coll) true (<= n 0))
    coll
    (recur (- n 1) (rest coll))))

(defn nth-count [coll]
  (reduce (fn [acc _] (+ acc 1)) 0 coll))

(defn last [coll]
  (if (empty? (rest coll))
    (first coll)
    (recur (rest coll))))

(defn second [coll] (first (rest coll)))

(defn concat [a b]
  (reduce conj (vec b) (reverse a)))

(defn every? [pred coll]
  (cond
    (empty? coll) true
    (pred (first coll)) (recur pred (rest coll))
    :else false))

(defn some [pred coll]
  (cond
    (empty? coll) nil
    (pred (first coll)) (first coll)
    :else (recur pred (rest coll))))

(defn constantly [x] (fn [& _] x))

(defn inc-by [n] (fn [x] (+ x n)))

(defn sum [coll] (reduce + 0 coll))

(defn max-of [coll] (reduce max coll))

(defn min-of [coll] (reduce min coll))

(defn count-if [pred coll]
  (reduce (fn [acc x] (if (pred x) (+ acc 1) acc)) 0 coll))
