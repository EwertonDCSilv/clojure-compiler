(ns io.clojure_core.with_out_str.boundary)
(defn -main [] (do (with-out-str nil) (println :ok)))
(-main)
