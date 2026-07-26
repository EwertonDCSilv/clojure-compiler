(ns closures.core)

;; Funções de primeira classe e closures compiladas nativamente (sem JVM).

;; HOF: recebem e chamam funções indiretamente.
(defn my-map [f coll]
  (if (empty? coll)
    (list)
    (cons (f (first coll)) (my-map f (rest coll)))))

(defn my-reduce [f acc coll]
  (if (empty? coll)
    acc
    (recur f (f acc (first coll)) (rest coll))))

;; Closure que captura o ambiente léxico.
(defn adder [n]
  (fn [x] (+ x n)))

;; Composição via closures aninhadas (captura transitiva).
(defn compose [f g]
  (fn [x] (f (g x))))

(defn dobro [x] (* x 2))
(defn mais1 [x] (+ x 1))

(defn -main []
  (println "quadrados:" (my-map (fn [x] (* x x)) (list 1 2 3 4 5)))
  (println "soma:" (my-reduce (fn [a b] (+ a b)) 0 (list 1 2 3 4 5 6 7 8 9 10)))
  (let [add10 (adder 10)]
    (println "add10 de 5:" (add10 5)))
  (let [dobro-e-mais1 (compose mais1 dobro)]
    (println "(inc (dobro 20)):" (dobro-e-mais1 20))))

(-main)
