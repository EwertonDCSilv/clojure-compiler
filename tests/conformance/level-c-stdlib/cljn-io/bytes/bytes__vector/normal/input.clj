(ns io.cljn_io.bytes__vector.normal)
(defn -main [] (do (cljn.io/bytes->vector (cljn.io/bytes [0 127 255])) (println :ok)))
(-main)
