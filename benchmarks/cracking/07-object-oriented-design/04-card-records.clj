(ns cracking.objects.cards)

(defrecord Card [rank suit])

(defn card-score [card]
  (+ (* (:suit card) 20) (min (:rank card) 10)))

(defn hand-score [cards]
  (reduce (fn [total card] (+ total (card-score card))) 0 cards))

(defn benchmark [rounds]
  (let [hand (list (->Card 1 0)
                   (->Card 13 1)
                   (->Card 7 2)
                   (->Card 11 3)
                   (->Card 5 0))]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (hand-score hand)))
        total))))

(defn -main [] (println (benchmark 8000)))
(-main)
