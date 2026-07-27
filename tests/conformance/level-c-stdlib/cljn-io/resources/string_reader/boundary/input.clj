(ns io.cljn_io.string_reader.boundary)
(defn -main [] (do (cljn.io/close! (cljn.io/string-reader "")) (println :ok)))
(-main)
