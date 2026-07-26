(ns factorial.core)

(defn fatorial-acc [n acc]
  (if (<= n 1)
    acc
    (fatorial-acc (- n 1) (* acc n))))

(defn loopar [n times]
  (if (<= times 0)
    1
    (do
      (fatorial-acc n 1)
      (loopar n (- times 1)))))

(defn -main []
  (println "fatorial 20 =" (fatorial-acc 20 1))
  (println "repeticoes 100x =>" (loopar 20 100)))

(-main)
