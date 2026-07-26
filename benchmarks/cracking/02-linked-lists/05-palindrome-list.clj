(ns cracking.lists.palindrome)

(defn palindrome-list? [xs]
  (= xs (reverse xs)))

(defn benchmark [rounds]
  (let [yes (list 1 2 3 4 3 2 1)
        no (list 1 2 3 4 5 6 7)]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n)
               (+ total
                  (if (palindrome-list? yes) 1 0)
                  (if (palindrome-list? no) 0 1)))
        total))))

(defn -main [] (println (benchmark 4000)))
(-main)
