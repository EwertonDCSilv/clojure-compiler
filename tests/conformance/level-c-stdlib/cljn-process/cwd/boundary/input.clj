(ns io.cljn_process.cwd.boundary)
(defn -main [] (do (cljn.io/absolute? (cljn.process/cwd)) (println :ok)))
(-main)
