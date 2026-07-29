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

(defn- a-byte?
  "True when `x` is an integer in the inclusive 0..255 byte range."
  [x]
  (and (int? x) (not (neg? x)) (<= x 255)))

;; A path value is a two-element vector `[:cljn-path "the/string"]`. `path`
;; is the only constructor; the path-algebra functions require this shape so a
;; bare string is rejected with `:invalid-input`, mirroring java.nio's Path type.

(defn- path-value?
  "True when `x` is a path value produced by `path`."
  [x]
  (and (vector? x) (= 2 (count x)) (= (nth x 0) :cljn-path)))

(defn- path-str
  "Returns the underlying string of a path value."
  [p]
  (nth p 1))

(defn bytes
  "Builds an immutable byte array from a vector of 0..255 integers."
  [v]
  (if (and (vector? v) (every? a-byte? v)) (bytes-of-vec v) (fail)))

(defn bytes?
  "Returns true when `b` is an immutable byte array."
  [b]
  (bytes? b))

(defn bytes->string
  "Decodes a byte array as UTF-8 text, raising on invalid UTF-8."
  [b]
  (if (and (bytes? b) (valid-utf8? b)) (bytes->string b) (fail)))

(defn bytes->vector
  "Returns a vector of the 0..255 integers in a byte array."
  [b]
  (if (bytes? b) (bytes->vec b) (fail)))

(defn string->bytes
  "Encodes a string as its UTF-8 byte array."
  [s]
  (if (string? s) (bytes s) (fail)))

(defn byte-count
  "Returns the number of bytes in a byte array."
  [b]
  (if (bytes? b) (count b) (fail)))

(defn path
  "Coerces a string into a path value. Path-algebra functions require a value
  produced here rather than a bare string."
  [s]
  (if (string? s) [:cljn-path s] (fail)))

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
  "Returns the final segment of a path value."
  [p]
  (if (path-value? p) (file-name (path-str p)) (fail)))

(defn parent
  "Returns the parent of a path value, or nil at the root."
  [p]
  (if (path-value? p) (parent (path-str p)) (fail)))

(defn join
  "Joins a base path value with additional string segments."
  [base & segments]
  (if (path-value? base)
    (reduce (fn [acc s] (path-join acc s)) (path-str base) segments)
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

;; --- byte streams ------------------------------------------------------------

(defn byte-input-stream
  "Returns a byte input stream over an immutable byte array."
  [b]
  (if (bytes? b) (byte-input-stream b) (fail)))

(defn byte-output-stream
  "Returns a byte output stream; an optional non-negative capacity hint is
  accepted."
  [& options]
  (if (or (empty? options) (not (neg? (first options))))
    (byte-output-stream)
    (fail)))

(defn read-bytes
  "Reads up to `n` bytes from an open byte input stream."
  [s n]
  (if (and (byte-input? s) (not (stream-closed s)) (int? n) (not (neg? n)))
    (read-bytes s n)
    (fail)))

(defn write-bytes!
  "Writes byte array `b` to an open byte output stream."
  [s b]
  (if (and (byte-output? s) (not (stream-closed s)) (bytes? b))
    (write-bytes! s b)
    (fail)))

(defn output-bytes
  "Returns the accumulated bytes of a byte output stream."
  [s]
  (if (byte-output? s) (output-bytes s) (fail)))

(defn read-block!
  "Reads up to `n` items (characters or bytes) from an open reader."
  [s n]
  (if (and (reader? s) (not (stream-closed s)) (int? n) (not (neg? n)))
    (read-block! s n)
    (fail)))

;; --- random access -----------------------------------------------------------

(defn seek!
  "Seeks an open file reader to absolute byte offset `n`."
  [s n]
  (if (and (file-reader? s) (not (stream-closed s)) (int? n) (not (neg? n)))
    (seek-file s n)
    (fail)))

(defn truncate!
  "Truncates an open file writer to `n` bytes."
  [s n]
  (if (and (file-writer? s) (not (stream-closed s)) (int? n) (not (neg? n)))
    (truncate-file s n)
    (fail)))

(defn position
  "Returns the current byte position of an open file reader."
  [s]
  (if (and (file-reader? s) (not (stream-closed s)))
    (position-file s)
    (fail)))

;; --- symbolic links and copy -------------------------------------------------

(defn create-symlink!
  "Creates a symbolic link `linkpath` pointing at `target`."
  [target linkpath]
  (if (and (string? target) (string? linkpath))
    (guard (fn [] (create-symlink target linkpath)))
    (fail)))

(defn read-link
  "Returns the target of a symbolic link."
  [p]
  (if (string? p) (guard (fn [] (read-link p))) (fail)))

(defn symlink?
  "Returns true when the path names a symbolic link."
  [p]
  (if (string? p) (native-symlink? p) (fail)))

(defn copy!
  "Copies the file at `src` to `dst`."
  [src dst]
  (if (and (string? src) (string? dst))
    (guard (fn [] (spit-bytes dst (slurp-bytes src))))
    (fail)))

;; --- path algebra ------------------------------------------------------------

(defn absolute?
  "Returns true when the path value is absolute (leading `/`)."
  [p]
  (if (path-value? p) (path-absolute (path-str p)) (fail)))

(defn normalize
  "Lexically normalizes a path value, collapsing `.` and `..` without touching
  the filesystem; returns a path value."
  [p]
  (if (path-value? p) (path (path-normalize (path-str p))) (fail)))

(defn real-path
  "Resolves a path value to its canonical form, following symlinks and requiring
  every component to exist; returns a path value."
  [p]
  (if (path-value? p) (guard (fn [] (path (real-path (path-str p))))) (fail)))
