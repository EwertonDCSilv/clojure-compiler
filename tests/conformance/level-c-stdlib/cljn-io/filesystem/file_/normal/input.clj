(ns io.cljn_io.file_.normal)
(defn -main [] (do (cljn.io/file? "input.txt") (println :ok)))
(-main)
