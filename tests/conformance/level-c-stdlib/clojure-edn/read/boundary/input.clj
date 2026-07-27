(ns io.clojure_edn.read.boundary)
(defn -main [] (do (clojure.edn/read {:eof :done} (cljn.io/string-reader "")) (println :ok)))
(-main)
