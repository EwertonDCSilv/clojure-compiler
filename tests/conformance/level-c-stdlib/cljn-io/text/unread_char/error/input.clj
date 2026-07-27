(ns io.cljn_io.unread_char.error)
(defn -main []
(try
(do (cljn.io/unread-char (cljn.io/string-reader "") \a) (println :unexpected))
(catch cljn.io/IOException error
(println (:kind (ex-data error))))))
(-main)
