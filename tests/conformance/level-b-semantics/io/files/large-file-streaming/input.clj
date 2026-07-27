(ns b.io.large-file)
(defn -main []
  (with-open [in (cljn.io/input-stream "large.bin")
              out (cljn.io/output-stream "copy.bin")]
    (loop [total 0]
      (if-let [chunk (cljn.io/read-bytes in 65536)]
        (do (cljn.io/write-bytes! out chunk)
            (recur (+ total (cljn.io/byte-count chunk))))
        (println total)))))
(-main)
