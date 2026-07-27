(ns io.clojure_core.with_out_str.normal)
(defn -main [] (do (with-out-str (print "alpha")) (println :ok)))
(-main)
