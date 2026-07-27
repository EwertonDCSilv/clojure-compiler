(ns io.cljn_io.close_.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/string-reader "x")) (println :ok)))
(-main)
