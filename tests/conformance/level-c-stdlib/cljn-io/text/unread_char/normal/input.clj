(ns io.cljn_io.unread_char.normal)
(defn -main [] (do (let [r (cljn.io/string-reader "ab")] (cljn.io/unread-char r (cljn.io/read-char r))) (println :ok)))
(-main)
