(ns native.core)

;; Subconjunto compilável (Fase 4/5 slice): valores tagged (int, nil, bool,
;; string, lista). defn de aridade fixa, if, do, let, recursão direta,
;; aritmética inteira, comparações, strings e listas via runtime.

(defn fib [n]
  (if (< n 2)
    n
    (+ (fib (- n 1)) (fib (- n 2)))))

(defn fatorial [n]
  (if (<= n 1)
    1
    (* n (fatorial (- n 1)))))

;; Constrói a lista (0 1 2 ... n) por recursão direta (sem loop/recur).
(defn ate [n acc]
  (if (< n 0)
    acc
    (ate (dec n) (cons n acc))))

(defn saudacao [nome]
  (str "Olá, " nome "! Você tem " (count nome) " letras no nome."))

(defn -main []
  (println "Hello from native Clojure")
  (println "fib 10 =" (fib 10))
  (println "fatorial 10 =" (fatorial 10))
  (println "lista 0..8 =" (ate 8 (list)))
  (println "primeiro =" (first (ate 8 (list))) "| contagem =" (count (ate 8 (list))))
  (println (saudacao "Clojure"))
  (println "igualdade de listas:" (= (list 1 2 3) (cons 1 (cons 2 (cons 3 (list)))))))

(-main)
