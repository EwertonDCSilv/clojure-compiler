(ns io.cljn_io.create_symlink_.normal)
(defn -main [] (do (cljn.io/create-symlink! "input.txt" "link.txt") (println :ok)))
(-main)
