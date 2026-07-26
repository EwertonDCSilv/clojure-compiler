(ns c.identity)
(defn -main [] (println (identity 42) (identity nil) (identity [:a :b])))
(-main)
