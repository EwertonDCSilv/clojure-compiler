(ns io.cljn_io.truncate_.boundary)
(defn -main [] (do (with-open [s (cljn.io/output-stream "created.bin")] (cljn.io/truncate! s 0)) (println :ok)))
(-main)
