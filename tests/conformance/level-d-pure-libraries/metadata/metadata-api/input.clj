(ns d.metadata-api)
(defn -main []
(println (:role (meta (with-meta [] {:role :data})))))
(-main)
