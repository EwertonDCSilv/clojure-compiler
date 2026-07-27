(ns io.clojure_core.newline.boundary)
(defn -main [] (do (binding [*flush-on-newline* false] (newline)) (println :ok)))
(-main)
