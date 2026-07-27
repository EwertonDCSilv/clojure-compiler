(ns io.clojure_core.with_open.boundary)
(defn -main [] (do (with-open [] nil) (println :ok)))
(-main)
