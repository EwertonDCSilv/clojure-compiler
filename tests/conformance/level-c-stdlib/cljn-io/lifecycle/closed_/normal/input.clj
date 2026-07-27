(ns io.cljn_io.closed_.normal)
(defn -main [] (do (let [r (cljn.io/string-reader "x")] (cljn.io/close! r) (cljn.io/closed? r)) (println :ok)))
(-main)
