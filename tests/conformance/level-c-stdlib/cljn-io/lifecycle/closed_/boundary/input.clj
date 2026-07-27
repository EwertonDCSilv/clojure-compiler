(ns io.cljn_io.closed_.boundary)
(defn -main [] (do (cljn.io/closed? (cljn.io/string-reader "")) (println :ok)))
(-main)
