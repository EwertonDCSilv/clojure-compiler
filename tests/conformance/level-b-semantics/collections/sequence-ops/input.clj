(ns b.seq)
(defn -main [] (println (first [1 2]) (rest [1 2]) (empty? []) (count "abc") (count nil)))
(-main)
