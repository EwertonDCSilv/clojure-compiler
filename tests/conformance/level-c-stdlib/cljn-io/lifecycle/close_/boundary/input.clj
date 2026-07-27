(ns io.cljn_io.close_.boundary)
(defn -main [] (do (let [r (cljn.io/string-reader "x")] (cljn.io/close! r) (cljn.io/close! r)) (println :ok)))
(-main)
