(ns io.cljn_process.getenv.normal)
(defn -main [] (do (cljn.process/getenv "CLJN_CONFORMANCE_VALUE") (println :ok)))
(-main)
