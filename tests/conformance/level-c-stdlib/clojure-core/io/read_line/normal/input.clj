(ns io.clojure_core.read_line.normal)
(defn -main [] (do (binding [*in* (cljn.io/string-reader "alpha\nbeta\n")] (read-line)) (println :ok)))
(-main)
