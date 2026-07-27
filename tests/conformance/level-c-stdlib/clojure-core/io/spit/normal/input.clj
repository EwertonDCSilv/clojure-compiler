(ns io.clojure_core.spit.normal)
(defn -main [] (do (spit "created.txt" "hello\n") (println :ok)))
(-main)
