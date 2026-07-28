(ns io.clojure_core.read_string.normal)
(defn -main [] (println (= {:answer 42} (read-string "{:answer 42}"))))
(-main)
