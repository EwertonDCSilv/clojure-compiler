(ns factorial.core)

(defn fatorial-acc [n acc]
  (if (<= n 1)
    acc
    (recur (- n 1) (* acc n))))

(defn loopar [n times]
  (loop [times times]
    (if (<= times 0)
      1
      (do
        (fatorial-acc n 1)
        (recur (- times 1))))))

(defn -main []
  (println "fatorial 20 =" (fatorial-acc 20 1))
  (println "repeticoes 10000x =>" (loopar 20 10000)))

(-main)
