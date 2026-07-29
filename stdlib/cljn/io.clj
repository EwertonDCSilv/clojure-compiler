;; Compiler-owned native I/O namespace (issue #103).
;;
;; `cljn.io` is always-available sugar: referencing a qualified name such as
;; `cljn.io/exists?` auto-loads this module (no `:require`). Each function
;; validates its arguments and throws a `cljn.io/IOException`-shaped data map
;; `{:kind :invalid-input}` on bad input, so error handlers read `(:kind (ex-data e))`.
;; Paths are plain strings in this subset. Functions delegate to the low-level
;; ADR-0007 runtime primitives; stream state and byte-stream constructors are
;; added incrementally.
(ns cljn.io)

(defn- fail
  "Throws the stable invalid-input payload used by this native subset."
  []
  (throw {:kind :invalid-input}))

(defn- guard
  "Runs thunk `f`, translating any native failure into `:invalid-input`."
  [f]
  (try (f) (catch IOException _error (fail))))

(defn path
  "Coerces a string to a filesystem path (strings are paths in this subset)."
  [s]
  (if (string? s) s (fail)))

(defn exists?
  "Returns true when the path exists."
  [p]
  (if (string? p) (file-exists? p) (fail)))

(defn directory?
  "Returns true when the path is a directory."
  [p]
  (if (string? p) (directory? p) (fail)))

(defn file?
  "Returns true when the path is a regular file."
  [p]
  (if (string? p) (file? p) (fail)))

(defn file-name
  "Returns the final path segment."
  [p]
  (if (string? p) (file-name p) (fail)))

(defn parent
  "Returns the parent path, or nil at the root."
  [p]
  (if (string? p) (parent p) (fail)))

(defn join
  "Joins a base path with additional string segments."
  [base & segments]
  (if (string? base)
    (reduce (fn [acc s] (path-join acc s)) base segments)
    (fail)))

(defn create-directory!
  "Creates a single directory. Optional trailing options are accepted; creating
  an existing directory is an error."
  [p & options]
  (if (string? p)
    (if (exists? p) (fail) (mkdir p))
    (fail)))

(defn create-directories!
  "Creates a directory and any missing parents. Optional trailing options are
  accepted."
  [p & options]
  (if (string? p) (guard (fn [] (mkdirs p))) (fail)))

(defn delete!
  "Deletes a file or empty directory. With a trailing option, a missing target
  is ignored instead of raising."
  [p & options]
  (if (string? p)
    (if (exists? p)
      (guard (fn [] (delete-file p)))
      (if (empty? options) (fail) nil))
    (fail)))

(defn list
  "Returns the entries of a directory."
  [p]
  (if (and (string? p) (directory? p)) (list-dir p) (fail)))

(defn move!
  "Renames a source path to a destination path."
  [src dst]
  (if (and (string? src) (string? dst) (file-exists? src))
    (rename src dst)
    (fail)))

;; --- streams -----------------------------------------------------------------

(defn- open-reader?
  "True when `r` is an open reader handle."
  [r]
  (and (reader? r) (not (stream-closed r))))

(defn- open-writer?
  "True when `w` is an open writer handle."
  [w]
  (and (writer? w) (not (stream-closed w))))

(defn- open-in
  "Opens a file path for reading, raising `:invalid-input` on failure."
  [p]
  (if (string? p) (guard (fn [] (reader p))) (fail)))

(defn- open-out
  "Opens a file path for writing, raising `:invalid-input` on failure."
  [p]
  (if (string? p) (guard (fn [] (writer p))) (fail)))

(defn reader
  "Opens a file path as a character reader; trailing options are accepted."
  [p & options]
  (open-in p))

(defn input-stream
  "Opens a file path as a binary input stream (a reader in this subset)."
  [p & options]
  (open-in p))

(defn writer
  "Opens a file path as a character writer; trailing options are accepted."
  [p & options]
  (open-out p))

(defn output-stream
  "Opens a file path as a binary output stream (a writer in this subset)."
  [p & options]
  (open-out p))

(defn string-reader
  "Returns a reader over an in-memory string."
  [s]
  (if (string? s) (string-reader s) (fail)))

(defn string-writer
  "Returns an in-memory string writer; an optional non-negative capacity hint is
  accepted."
  [& options]
  (if (or (empty? options) (not (neg? (first options))))
    (string-writer)
    (fail)))

(defn writer-string
  "Returns the accumulated text of a string writer."
  [w]
  (if (writer? w) (writer->string w) (fail)))

(defn read-char
  "Reads one character from an open reader, or nil at end of input."
  [r]
  (if (open-reader? r) (read-char-from r) (fail)))

(defn read-line
  "Reads one line (without the newline) from an open reader, or nil at end."
  [r]
  (if (open-reader? r) (read-line-from r) (fail)))

(defn unread-char
  "Pushes character `ch` back into open reader `r` for the next read, requiring a
  prior read."
  [r ch]
  (if (and (open-reader? r) (unread-char-to r ch)) nil (fail)))

(defn write!
  "Writes string `s` to open writer `w`."
  [w s]
  (if (and (open-writer? w) (string? s)) (write-to w s) (fail)))

(defn flush!
  "Flushes an open writer."
  [w]
  (if (open-writer? w) (flush-writer w) (fail)))

(defn close!
  "Closes a file or string reader/writer handle; closing twice is allowed.
  Standard streams cannot be closed."
  [x]
  (if (closeable? x) (close x) (fail)))

(defn closed?
  "Returns true when the stream handle is closed."
  [x]
  (let [c (stream-closed x)]
    (if (nil? c) (fail) c)))
