(ns io.clojure_core.with_out_str.boundary)
(defn -main [] (println (= "" (with-out-str nil))))
(-main)
