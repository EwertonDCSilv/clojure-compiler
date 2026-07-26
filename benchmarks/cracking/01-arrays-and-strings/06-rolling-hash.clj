(ns cracking.arrays.rolling-hash)

;; Inteiros representam caracteres porque o subconjunto compilado ainda não expõe
;; operações de indexação de strings.
(defn rolling-hash [codes]
  (loop [i 0 hash 7]
    (if (< i (count codes))
      (recur (inc i) (mod (+ (* hash 31) (nth codes i)) 1000003))
      hash)))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (rolling-hash [99 108 111 106 117 114 101])))
      total)))

(defn -main [] (println (benchmark 4000)))
(-main)
