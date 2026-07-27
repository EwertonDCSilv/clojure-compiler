(ns io.cljn_io.write_.boundary)
(defn -main [] (do (cljn.io/write! (cljn.io/string-writer) "") (println :ok)))
(-main)
