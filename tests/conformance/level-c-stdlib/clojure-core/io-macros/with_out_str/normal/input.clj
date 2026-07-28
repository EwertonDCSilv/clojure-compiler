(ns io.clojure_core.with_out_str.normal)
(defn -main [] (println (= "alpha" (with-out-str (print "alpha")))))
(-main)
