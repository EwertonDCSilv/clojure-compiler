(ns io.cljn_io.read_char.normal)
(defn -main [] (do (cljn.io/read-char (cljn.io/string-reader "λ")) (println :ok)))
(-main)
