(ns io.cljn_io.real_path.normal)
(defn -main [] (do (cljn.io/real-path (cljn.io/path "input.txt")) (println :ok)))
(-main)
