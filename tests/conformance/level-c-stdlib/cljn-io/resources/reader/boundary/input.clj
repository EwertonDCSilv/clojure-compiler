(ns io.cljn_io.reader.boundary)
(defn -main [] (do (cljn.io/close! (cljn.io/reader "empty.txt")) (println :ok)))
(-main)
