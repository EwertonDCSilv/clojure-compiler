(ns io.cljn_io.read_char.boundary)
(defn -main [] (do (cljn.io/read-char (cljn.io/string-reader "")) (println :ok)))
(-main)
