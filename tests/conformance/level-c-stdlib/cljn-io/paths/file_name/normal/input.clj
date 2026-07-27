(ns io.cljn_io.file_name.normal)
(defn -main [] (do (cljn.io/file-name (cljn.io/path "tree/a.txt")) (println :ok)))
(-main)
