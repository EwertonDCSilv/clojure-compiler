(ns io.cljn_io.writer_string.boundary)
(defn -main [] (do (cljn.io/writer-string (cljn.io/string-writer)) (println :ok)))
(-main)
