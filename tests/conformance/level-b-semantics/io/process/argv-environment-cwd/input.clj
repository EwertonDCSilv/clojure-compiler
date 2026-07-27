(ns b.io.process)
(defn -main []
  (println *command-line-args*)
  (println (cljn.process/getenv "CLJN_CONFORMANCE_VALUE"))
  (println (cljn.io/absolute? (cljn.process/cwd))))
(-main)
