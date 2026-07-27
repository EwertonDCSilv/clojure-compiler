(ns io.cljn_io.byte_count.boundary)
(defn -main [] (do (cljn.io/byte-count (cljn.io/bytes [])) (println :ok)))
(-main)
