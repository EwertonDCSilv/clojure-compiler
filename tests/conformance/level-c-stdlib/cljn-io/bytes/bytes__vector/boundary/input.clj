(ns io.cljn_io.bytes__vector.boundary)
(defn -main [] (do (cljn.io/bytes->vector (cljn.io/bytes [])) (println :ok)))
(-main)
