;; core_compiled.clj — parte de clojure.core escrita no subconjunto COMPILÁVEL,
;; pré-carregada em todo `clojure-native build` (ADR-0005: bootstrap progressivo).
;; Usa apenas o que o codegen nativo suporta: defn/fn/if/cond/let/loop/recur,
;; primitivas e coleções, incluindo variádicos quando necessário.

(ns clojure.core)

(defn zero? [n] (= n 0))
(defn pos? [n] (> n 0))
(defn neg? [n] (< n 0))
(defn even? [n] (= (mod n 2) 0))
(defn odd? [n] (not (even? n)))
(defn max [x & more] (reduce (fn [a b] (if (> a b) a b)) x more))
(defn min [x & more] (reduce (fn [a b] (if (< a b) a b)) x more))

(defn reduce [f acc coll]
  (if (empty? coll)
    acc
    (recur f (f acc (first coll)) (rest coll))))

(defn map [f coll]
  (if (empty? coll)
    (list)
    (cons (f (first coll)) (map f (rest coll)))))

(defn filter [pred coll]
  (cond
    (empty? coll) (list)
    (pred (first coll)) (cons (first coll) (filter pred (rest coll)))
    :else (filter pred (rest coll))))

(defn remove [pred coll]
  (filter (fn [x] (not (pred x))) coll))

(defn reverse [coll]
  (reduce (fn [acc x] (cons x acc)) (list) coll))

(defn take [n coll]
  (if (if (empty? coll) true (<= n 0))
    (list)
    (cons (first coll) (take (dec n) (rest coll)))))

(defn drop [n coll]
  (if (if (empty? coll) true (<= n 0))
    coll
    (recur (dec n) (rest coll))))

(defn range [n]
  (loop [i 0 acc (list)]
    (if (< i n)
      (recur (inc i) (cons i acc))
      (reverse acc))))

(defn into [to from]
  (reduce conj to from))

(defn mapv [f coll]
  (reduce (fn [acc x] (conj acc (f x))) [] coll))

(defn every? [pred coll]
  (cond
    (empty? coll) true
    (pred (first coll)) (recur pred (rest coll))
    :else false))

(defn some [pred coll]
  (if (empty? coll)
    nil
    (let [r (pred (first coll))]
      (if r r (some pred (rest coll))))))

(defn comp [f g]
  (fn [x] (f (g x))))

(defn identity [x] x)

(defn second [coll] (first (rest coll)))

(defn last [coll]
  (if (empty? (rest coll))
    (first coll)
    (recur (rest coll))))

(defn concat [a b]
  (reduce (fn [acc x] (cons x acc)) b (reverse a)))

(defn mapcat [f coll]
  (reduce (fn [acc x] (concat acc (f x))) (list) coll))

(defn count-if [pred coll]
  (reduce (fn [acc x] (if (pred x) (inc acc) acc)) 0 coll))
