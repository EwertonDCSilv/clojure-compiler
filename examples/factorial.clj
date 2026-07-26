(ns factorial.core)

(defn fatorial [n]
  (if (<= n 1)
    1
    (* n (fatorial (- n 1)))))

(defn loopar [n times]
  (if (<= times 0)
    1
    (do
      (fatorial n)
      (loopar n (- times 1)))))

(defn -main []
  (println "fatorial 20 =" (fatorial 20))
  (println "repeticoes 100000x =>" (loopar 20 100000)))

(-main)
