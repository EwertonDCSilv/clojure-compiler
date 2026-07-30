;; Compiler-owned native `clojure.edn` namespace.
;;
;; Always-available sugar like `cljn.io`: a qualified reference to
;; `clojure.edn/read-string` auto-loads this module. The native subset reads the
;; same EDN grammar as `clojure.core/read-string` (no reader eval, no custom tag
;; dispatch yet); malformed or unsupported input throws `{:kind :invalid-input}`.
(ns clojure.edn)

(defn- fail
  "Throws the stable invalid-input payload used by this native subset."
  []
  (throw {:kind :invalid-input}))

#_{:clj-kondo/ignore [:unresolved-symbol :type-mismatch]}
(defn read-string
  "Reads one EDN value from a string. An optional leading options map is accepted
  and ignored in this subset. Malformed or unsupported input raises
  `:invalid-input`."
  ([s] (if (string? s) (read-string s) (fail)))
  ([_options s] (if (string? s) (read-string s) (fail))))

#_{:clj-kondo/ignore [:unresolved-symbol :type-mismatch]}
(defn read
  "Reads one EDN value from a string reader. With an options map, an exhausted
  reader yields the map's `:eof` value; without one it raises `:invalid-input`."
  ([reader] (if (reader-eof? reader) (fail) (read-from reader)))
  ([options reader] (if (reader-eof? reader) (:eof options) (read-from reader))))
