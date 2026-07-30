;; Compiler-owned native process namespace (issue #103).
;;
;; Always-available sugar like `cljn.io`: a qualified reference to
;; `cljn.process/getenv` auto-loads this module. Invalid input throws a
;; `{:kind :invalid-input}` data map.
(ns cljn.process)

(defn- fail
  "Throws the stable invalid-input payload used by this native subset."
  []
  (throw {:kind :invalid-input}))

(defn getenv
  "Returns the value of environment variable `name`, or nil when unset."
  [name]
  (if (string? name) (getenv name) (fail)))

(defn cwd
  "Returns the process working directory as a `cljn.io` path value, so it composes
  with the path-algebra functions. Any argument is rejected with `:invalid-input`."
  [& args]
  (if (empty? args) (cljn.io/path (process-cwd)) (fail)))

(defn environment
  "Returns an immutable map of every environment variable name to its value. Any
  argument is rejected with `:invalid-input`."
  [& args]
  (if (empty? args) (process-environment) (fail)))
