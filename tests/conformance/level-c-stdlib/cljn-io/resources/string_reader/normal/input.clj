(ns io.cljn_io.string_reader.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/string-reader "abc")) (println :ok)))
(-main)
