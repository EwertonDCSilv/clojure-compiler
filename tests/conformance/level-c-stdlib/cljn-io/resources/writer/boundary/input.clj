(ns io.cljn_io.writer.boundary)
(defn -main [] (do (cljn.io/close! (cljn.io/writer "created.txt" :append true)) (println :ok)))
(-main)
