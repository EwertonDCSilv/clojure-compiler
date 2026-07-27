(ns e.runtime-require)
(defn -main []
(require 'clojure.set)
(println (clojure.set/union #{1 2} #{2 3})))
(-main)
