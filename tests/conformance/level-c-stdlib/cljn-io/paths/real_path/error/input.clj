(ns io.cljn_io.real_path.error)
(defn -main []
(try
(do (cljn.io/real-path (cljn.io/path "missing")) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
