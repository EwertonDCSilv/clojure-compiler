(ns io.cljn_io.truncate_.error)
(defn -main []
(try
(do (with-open [s (cljn.io/output-stream "created.bin")] (cljn.io/truncate! s -1)) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
