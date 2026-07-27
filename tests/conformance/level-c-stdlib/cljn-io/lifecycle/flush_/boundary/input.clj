(ns io.cljn_io.flush_.boundary)
(defn -main [] (do (cljn.io/flush! *out*) (println :ok)))
(-main)
