(ns io.cljn_io.unread_char.boundary)
(defn -main [] (do (let [r (cljn.io/string-reader "a")] (cljn.io/read-char r) (cljn.io/unread-char r \a)) (println :ok)))
(-main)
