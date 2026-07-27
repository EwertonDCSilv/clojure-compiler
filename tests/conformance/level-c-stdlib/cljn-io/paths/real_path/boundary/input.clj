(ns io.cljn_io.real_path.boundary)
(defn -main [] (do (cljn.io/real-path (cljn.io/path ".")) (println :ok)))
(-main)
