;; Compiler-owned native process namespace (issue #103).
;;
;; Always-available sugar like `cljn.io`: a qualified reference to
;; `cljn.process/getenv` auto-loads this module. Invalid input throws a
;; `{:kind :invalid-input}` data map. `cwd` and `environment` follow once their
;; backing runtime primitives land.
(ns cljn.process)

(defn- fail
  "Throws the stable invalid-input payload used by this native subset."
  []
  (throw {:kind :invalid-input}))

(defn getenv
  "Returns the value of environment variable `name`, or nil when unset."
  [name]
  (if (string? name) (getenv name) (fail)))
