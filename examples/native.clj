(ns native.core)

;; Subconjunto compilável (Fase 5 slice): defn de aridade fixa, if, let,
;; aritmética inteira, recursão direta e println (strings + inteiros).

(defn fib [n]
  (if (< n 2)
    n
    (+ (fib (- n 1)) (fib (- n 2)))))

(defn fatorial [n]
  (if (<= n 1)
    1
    (* n (fatorial (- n 1)))))

(defn -main []
  (println "Hello from native Clojure")
  (println "fib 10 =" (fib 10))
  (println "fatorial 10 =" (fatorial 10))
  (println "soma via let =" (let [a 20 b 22] (+ a b))))

(-main)
