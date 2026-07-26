(ns b.eq)
(defn -main [] (println (= [1 2] [1 2]) (= {:a 1 :b 2} {:b 2 :a 1}) (= #{1 2} #{2 1}) (= nil false)))
(-main)
