(ns io.clojure_core.read_line.boundary)
(defn -main [] (do (binding [*in* (cljn.io/string-reader "")] (read-line)) (println :ok)))
(-main)
