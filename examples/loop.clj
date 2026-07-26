(ns loop.core)

;; loop/recur compilado nativamente (backedge, sem crescer a pilha).

(defn soma-ate [n]
  (loop [i 0 acc 0]
    (if (> i n)
      acc
      (recur (inc i) (+ acc i)))))

;; recur direto para a fn (a fn é alvo de recur).
(defn conta [n acc]
  (if (= n 0)
    acc
    (recur (dec n) (inc acc))))

;; Constrói uma lista com loop/recur e a inverte no processo.
(defn faixa [n]
  (loop [i n acc (list)]
    (if (< i 0)
      acc
      (recur (dec i) (cons i acc)))))

(defn -main []
  (println "soma 0..100 =" (soma-ate 100))
  (println "conta 1000000 =" (conta 1000000 0))
  (println "faixa 10 =" (faixa 10))
  (println "tamanho =" (count (faixa 1000))))

(-main)
