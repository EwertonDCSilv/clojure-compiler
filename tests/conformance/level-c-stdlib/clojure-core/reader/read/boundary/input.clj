(ns io.clojure_core.read.boundary)
(defn -main [] (do (read {:eof :done} (cljn.io/string-reader "")) (println :ok)))
(-main)
