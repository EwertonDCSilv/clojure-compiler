(ns b.do)
(defn -main [] (println (do 1 2 3)) (do (print "a") (println "b")))
(-main)
