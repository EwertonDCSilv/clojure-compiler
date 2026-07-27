(ns b.transients.use-after-persistent)
(defn -main []
(println
(try
(let [value (transient [])]
(persistent! value)
(conj! value 1)
:unexpected)
(catch Exception error :caught))))
(-main)
