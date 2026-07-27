(ns io.cljn_io.read_link.boundary)
(defn -main [] (do (do (cljn.io/create-symlink! "missing" "dangling") (cljn.io/read-link "dangling")) (println :ok)))
(-main)
