(ns io.cljn_io.writer_string.normal)
(defn -main [] (do (let [w (cljn.io/string-writer)] (cljn.io/write! w "ação") (cljn.io/writer-string w)) (println :ok)))
(-main)
