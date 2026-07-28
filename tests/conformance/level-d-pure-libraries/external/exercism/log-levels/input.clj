;; Upstream: Exercism Clojure Track, concept exercise "log-levels".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/log-levels/.meta/exemplar.clj
;; License: MIT; see benchmarks/exercism/LICENSE.exercism and UPSTREAM.md.

(ns log-levels
  (:require [clojure.string :as str]))

(defn message
  "Takes a string representing a log line
   and returns its message with whitespace trimmed."
  [s]
  (str/trim (last (str/split s #":"))))

(defn log-level
  "Takes a string representing a log line
   and returns its level in lower-case."
  [s]
  (str/lower-case (subs (first (str/split s #"]")) 1)))

(defn reformat
  "Takes a string representing a log line and formats it
   with the message first and the log level in parentheses."
  [s]
  (str (message s) " (" (log-level s) ")"))

;; BEGIN LOCAL CONFORMANCE DRIVER

(println (message "[ERROR]: Invalid operation"))
(println (log-level "[WARNING]: Disk almost full"))
(println (reformat "[INFO]: Operation completed"))
