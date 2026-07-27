(ns io.cljn_io.byte_count.normal)
(defn -main [] (do (cljn.io/byte-count (cljn.io/bytes [1 2 3])) (println :ok)))
(-main)
