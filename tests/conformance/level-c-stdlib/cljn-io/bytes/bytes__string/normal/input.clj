(ns io.cljn_io.bytes__string.normal)
(defn -main [] (do (cljn.io/bytes->string (cljn.io/bytes [97 195 167 195 163 111])) (println :ok)))
(-main)
