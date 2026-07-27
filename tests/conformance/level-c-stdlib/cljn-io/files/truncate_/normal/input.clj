(ns io.cljn_io.truncate_.normal)
(defn -main [] (do (with-open [s (cljn.io/output-stream "created.bin")] (cljn.io/truncate! s 2)) (println :ok)))
(-main)
