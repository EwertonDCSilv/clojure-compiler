(ns io.cljn_io.reader.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/reader "input.txt")) (println :ok)))
(-main)
