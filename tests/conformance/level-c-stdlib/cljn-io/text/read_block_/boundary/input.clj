(ns io.cljn_io.read_block_.boundary)
(defn -main [] (do (cljn.io/read-block! (cljn.io/string-reader "") 0) (println :ok)))
(-main)
