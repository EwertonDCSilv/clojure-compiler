(ns b.truth)
(defn -main [] (println (if nil :bad :nil-false) (if false :bad :false-false) (if 0 :zero-true :bad) (if "" :string-true :bad)))
(-main)
