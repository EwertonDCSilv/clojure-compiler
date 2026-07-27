(ns io.cljn_process.getenv.boundary)
(defn -main [] (do (cljn.process/getenv "CLJN_CONFORMANCE_MISSING") (println :ok)))
(-main)
