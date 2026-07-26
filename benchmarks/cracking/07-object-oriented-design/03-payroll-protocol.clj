(ns cracking.objects.payroll)

(defprotocol Payable (monthly-cost [this]))
(defrecord Salaried [monthly])
(defrecord Hourly [rate hours])

(extend-type Salaried Payable
  (monthly-cost [this] (:monthly this)))

(extend-type Hourly Payable
  (monthly-cost [this] (* (:rate this) (:hours this))))

(defn payroll [workers]
  (reduce (fn [total worker] (+ total (monthly-cost worker))) 0 workers))

(defn benchmark [rounds]
  (let [workers (list (->Salaried 6000)
                      (->Hourly 45 120)
                      (->Salaried 7500)
                      (->Hourly 30 160))]
    (loop [n rounds total 0]
      (if (> n 0)
        (recur (dec n) (+ total (payroll workers)))
        total))))

(defn -main [] (println (benchmark 5000)))
(-main)
