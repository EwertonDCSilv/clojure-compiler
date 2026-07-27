(ns io.cljn_io.read_line.boundary)
(defn -main [] (do (cljn.io/read-line (cljn.io/string-reader "last")) (println :ok)))
(-main)
