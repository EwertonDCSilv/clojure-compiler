(ns io.cljn_io.read_line.normal)
(defn -main [] (do (cljn.io/read-line (cljn.io/string-reader "a\r\nb\n")) (println :ok)))
(-main)
