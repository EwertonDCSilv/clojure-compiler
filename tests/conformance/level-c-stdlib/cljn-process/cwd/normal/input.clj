(ns io.cljn_process.cwd.normal)
(defn -main [] (do (cljn.process/cwd) (println :ok)))
(-main)
