(ns io.clojure_core.spit.boundary)
(defn -main [] (do (spit "created.txt" "" :append true) (println :ok)))
(-main)
