(ns e.polymorphic-service)
(defprotocol Handler (handle [request]))
(defrecord AddRequest [left right])
(defrecord MultiplyRequest [left right])
(extend-type AddRequest Handler
(handle [request] (+ (:left request) (:right request))))
(extend-type MultiplyRequest Handler
(handle [request] (* (:left request) (:right request))))
(defn -main []
(println (mapv handle [(->AddRequest 20 22)
(->MultiplyRequest 6 7)])))
(-main)
