(ns io.clojure_core.with_out_str.error)
(defn -main []
(try
(do (with-out-str (throw (ex-info "boom" {}))) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
