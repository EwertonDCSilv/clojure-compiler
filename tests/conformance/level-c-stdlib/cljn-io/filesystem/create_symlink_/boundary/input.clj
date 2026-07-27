(ns io.cljn_io.create_symlink_.boundary)
(defn -main [] (do (cljn.io/create-symlink! "missing-target" "dangling") (println :ok)))
(-main)
