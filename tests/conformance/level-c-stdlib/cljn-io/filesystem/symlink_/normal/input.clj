(ns io.cljn_io.symlink_.normal)
(defn -main [] (do (cljn.io/symlink? "link.txt") (println :ok)))
(-main)
