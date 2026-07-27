(ns io.cljn_io.delete_.boundary)
(defn -main [] (do (cljn.io/delete! "missing.txt" :missing :ignore) (println :ok)))
(-main)
