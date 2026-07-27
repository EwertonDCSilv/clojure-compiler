(ns b.io.out-restored)
(defn -main []
  (let [capture (cljn.io/string-writer)]
    (binding [*out* capture] (print "captured"))
    (println "terminal")))
(-main)
