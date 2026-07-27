(ns io.cljn_io.create_directory_.normal)
(defn -main [] (do (cljn.io/create-directory! "new-dir") (println :ok)))
(-main)
