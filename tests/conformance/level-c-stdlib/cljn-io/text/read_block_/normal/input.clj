(ns io.cljn_io.read_block_.normal)
(defn -main [] (do (cljn.io/read-block! (cljn.io/string-reader "abcd") 4) (println :ok)))
(-main)
