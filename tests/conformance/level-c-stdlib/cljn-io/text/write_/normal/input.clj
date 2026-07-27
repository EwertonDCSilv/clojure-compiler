(ns io.cljn_io.write_.normal)
(defn -main [] (do (cljn.io/write! (cljn.io/string-writer) "ação") (println :ok)))
(-main)
