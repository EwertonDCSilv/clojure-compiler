(ns io.cljn_io.absolute_.normal)
(defn -main [] (do (cljn.io/absolute? (cljn.io/path "/tmp")) (println :ok)))
(-main)
