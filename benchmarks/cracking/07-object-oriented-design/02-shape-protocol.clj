(ns cracking.objects.shape-protocol)

(defprotocol ShapeMeasure (measure [this]))
(defrecord Rectangle [width height])
(defrecord Square [side])

(extend-type Rectangle ShapeMeasure
  (measure [this] (* (:width this) (:height this))))

(extend-type Square ShapeMeasure
  (measure [this] (* (:side this) (:side this))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n)
             (+ total
                (measure (->Rectangle 12 7))
                (measure (->Square 9))))
      total)))

(defn -main [] (println (benchmark 8000)))
(-main)
