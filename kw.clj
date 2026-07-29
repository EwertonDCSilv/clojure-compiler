(ns t.core)
(defn -main [] (println :cljn.error/domain (get {:cljn.error/domain :http} :cljn.error/domain)))
(-main)
