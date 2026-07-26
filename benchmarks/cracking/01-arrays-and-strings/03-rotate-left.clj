(ns cracking.arrays.rotate-left)

(defn rotate-left [xs shift]
  (let [size (count xs)]
    (loop [i 0 out []]
      (if (< i size)
        (recur (inc i) (conj out (nth xs (mod (+ i shift) size))))
        out))))

(defn checksum [xs]
  (reduce (fn [acc x] (+ (* acc 17) x)) 0 xs))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (checksum (rotate-left [11 22 33 44 55 66 77] (mod n 7)))))
      total)))

(defn -main [] (println (benchmark 2000)))
(-main)
