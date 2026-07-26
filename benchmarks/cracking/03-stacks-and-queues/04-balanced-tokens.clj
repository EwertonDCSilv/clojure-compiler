(ns cracking.stacks.balanced-tokens)

;; 1 abre um grupo e -1 fecha. Outros valores são conteúdo.
(defn balanced-tokens? [tokens]
  (loop [remaining tokens depth 0]
    (cond
      (< depth 0) false
      (empty? remaining) (= depth 0)
      (= (first remaining) 1) (recur (rest remaining) (inc depth))
      (= (first remaining) -1) (recur (rest remaining) (dec depth))
      :else (recur (rest remaining) depth))))

(defn benchmark [rounds]
  (let [valid (list 1 0 1 0 -1 1 -1 -1)
        invalid (list 1 -1 -1 1)]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total
                  (if (balanced-tokens? valid) 1 0)
                  (if (balanced-tokens? invalid) 0 1)))
        total))))

(defn -main [] (println (benchmark 10000)))
(-main)
