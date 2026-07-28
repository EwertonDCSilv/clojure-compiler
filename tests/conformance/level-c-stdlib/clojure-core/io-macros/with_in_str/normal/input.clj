(ns io.clojure_core.with_in_str.normal)
(defn -main [] (println (= "alpha" (with-in-str "alpha\n" (read-line)))))
(-main)
