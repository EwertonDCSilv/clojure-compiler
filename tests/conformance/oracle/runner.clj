(ns conformance.oracle
  (:import [java.io PushbackReader FileReader]))

(defn read-all [path]
  (with-open [reader (PushbackReader. (FileReader. path))]
    (let [eof (Object.)]
      (loop []
        (let [form (read {:eof eof} reader)]
          (when-not (identical? eof form)
            (binding [*print-meta* true]
              (prn form))
            (recur)))))))

(let [[mode path] *command-line-args*]
  (case mode
    "reader" (read-all path)
    "run" (load-file path)
    (throw (ex-info "unknown oracle mode" {:mode mode}))))
