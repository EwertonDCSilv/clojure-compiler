(ns io.cljn_io.file_name.boundary)
(defn -main [] (do (cljn.io/file-name (cljn.io/path "/")) (println :ok)))
(-main)
