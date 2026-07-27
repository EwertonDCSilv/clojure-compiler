(ns io.clojure_core.with_open.normal)
(defn -main [] (do (with-open [r (cljn.io/string-reader "x")] (cljn.io/read-char r)) (println :ok)))
(-main)
