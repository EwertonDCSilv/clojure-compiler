(ns io.cljn_io.flush_.normal)
(defn -main [] (do (cljn.io/flush! (cljn.io/string-writer)) (println :ok)))
(-main)
