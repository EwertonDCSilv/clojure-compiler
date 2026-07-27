(ns io.cljn_io.writer.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/writer "created.txt")) (println :ok)))
(-main)
