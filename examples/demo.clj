(ns demo.core)

;; Demonstra funções de ordem superior, recursão em tail (recur) e macros de core.

(defn factorial [n]
  (loop [i n acc 1]
    (if (<= i 1) acc (recur (- i 1) (* acc i)))))

(defn fib [n]
  (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))

(defn -main []
  (println "fatorial de 10 =" (factorial 10))
  (println "fib 10 =" (fib 10))
  (println "quadrados pares 0..9 =" (->> (range 10) (filter even?) (map (fn [x] (* x x)))))
  (println "soma 1..100 =" (reduce + 0 (range 1 101))))

(-main)
