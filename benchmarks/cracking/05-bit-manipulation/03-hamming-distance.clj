(ns cracking.bits.hamming-distance)

(defn hamming-distance [left right]
  (loop [a left b right distance 0]
    (if (and (= a 0) (= b 0))
      distance
      (recur (quot a 2)
             (quot b 2)
             (+ distance (if (= (mod a 2) (mod b 2)) 0 1))))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (hamming-distance 123456 654321)))
      total)))

(defn -main [] (println (benchmark 20000)))
(-main)
