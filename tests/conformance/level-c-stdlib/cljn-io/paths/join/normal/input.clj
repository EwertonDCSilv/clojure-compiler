(ns io.cljn_io.join.normal)
(defn -main [] (do (cljn.io/join (cljn.io/path "tree") "nested" "b.txt") (println :ok)))
(-main)
