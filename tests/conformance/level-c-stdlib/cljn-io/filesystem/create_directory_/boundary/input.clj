(ns io.cljn_io.create_directory_.boundary)
(defn -main [] (do (cljn.io/create-directory! "new-dir" :exists :ignore) (println :ok)))
(-main)
