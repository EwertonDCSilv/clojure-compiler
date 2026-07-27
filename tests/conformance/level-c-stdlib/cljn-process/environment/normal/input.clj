(ns io.cljn_process.environment.normal)
(defn -main [] (do (cljn.process/environment) (println :ok)))
(-main)
