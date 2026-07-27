(ns io.clojure_core.read.normal)
(defn -main [] (do (read (cljn.io/string-reader "{:answer 42}")) (println :ok)))
(-main)
