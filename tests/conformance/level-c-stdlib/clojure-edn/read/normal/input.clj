(ns io.clojure_edn.read.normal)
(defn -main [] (do (clojure.edn/read (cljn.io/string-reader "{:a [1 2]}")) (println :ok)))
(-main)
