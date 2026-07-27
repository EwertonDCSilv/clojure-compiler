(ns b.io.stderr)
(defn -main []
  (binding [*out* *err*] (println "problem"))
  (println "ok"))
(-main)
