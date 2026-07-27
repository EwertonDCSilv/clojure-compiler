(ns io.cljn_io.read_link.normal)
(defn -main [] (do (do (cljn.io/create-symlink! "input.txt" "link.txt") (cljn.io/read-link "link.txt")) (println :ok)))
(-main)
