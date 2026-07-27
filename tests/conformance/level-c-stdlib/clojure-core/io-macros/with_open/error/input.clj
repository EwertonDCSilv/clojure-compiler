(ns io.clojure_core.with_open.error)
(defn -main []
(try
(do (with-open [r (cljn.io/string-reader "x")] (throw (ex-info "boom" {}))) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
