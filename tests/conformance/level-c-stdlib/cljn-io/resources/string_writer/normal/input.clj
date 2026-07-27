(ns io.cljn_io.string_writer.normal)
(defn -main [] (do (cljn.io/close! (cljn.io/string-writer)) (println :ok)))
(-main)
