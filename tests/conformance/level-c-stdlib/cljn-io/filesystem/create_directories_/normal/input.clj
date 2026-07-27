(ns io.cljn_io.create_directories_.normal)
(defn -main [] (do (cljn.io/create-directories! "new/a/b") (println :ok)))
(-main)
