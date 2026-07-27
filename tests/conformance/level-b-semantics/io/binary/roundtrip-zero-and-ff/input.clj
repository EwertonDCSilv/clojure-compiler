(ns b.io.binary-roundtrip)
(defn -main []
  (let [out (cljn.io/byte-output-stream)]
    (cljn.io/write-bytes! out (cljn.io/bytes [0 255]))
    (println (cljn.io/bytes->vector (cljn.io/output-bytes out)))))
(-main)
