(ns b.when)
(defn -main [] (when true (print "yes")) (when false (print "no")) (println))
(-main)
