(ns io.cljn_io.bytes.normal)
(defn -main [] (do (cljn.io/bytes [0 1 255]) (println :ok)))
(-main)
