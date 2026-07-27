(ns io.clojure_edn.read_string.normal)
(defn -main [] (do (clojure.edn/read-string "#{1 2}") (println :ok)))
(-main)
