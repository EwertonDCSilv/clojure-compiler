(ns io.cljn_process.environment.boundary)
(defn -main [] (do (contains? (cljn.process/environment) "CLJN_CONFORMANCE_VALUE") (println :ok)))
(-main)
