(ns cormen.number.extended-euclid)

(defn extended-gcd [a b]
  (if (= b 0)
    [a 1 0]
    (let [result (extended-gcd b (mod a b))
          gcd (nth result 0)
          x1 (nth result 1)
          y1 (nth result 2)]
      [gcd y1 (- x1 (* (quot a b) y1))])))

(defn bezout-checksum [a b]
  (let [result (extended-gcd a b)]
    (+ (* 1000 (nth result 0))
       (* a (nth result 1))
       (* b (nth result 2)))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n)
             (+ checksum
                (bezout-checksum 240 46)
                (bezout-checksum 391 299)
                (bezout-checksum 1071 462)))
      checksum)))

(defn -main [] (println (benchmark 16000)))
(-main)
