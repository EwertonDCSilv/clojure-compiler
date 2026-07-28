;; Upstream: Exercism Clojure Track, concept exercise "annalyns-infiltration".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/annalyns-infiltration/.meta/exemplar.clj
;; License: MIT; attribution and local benchmark changes: ../UPSTREAM.md

(ns annalyns-infiltration)

(defn can-fast-attack?
  "Returns true if a fast-attack can be made, false otherwise."
  [knight-awake?]
  (not knight-awake?))

(defn can-spy?
  "Returns true if the kidnappers can be spied upon, false otherwise."
  [knight-awake? archer-awake? prisoner-awake?]
  (or knight-awake? archer-awake? prisoner-awake?))

(defn can-signal-prisoner?
  "Returns true if the prisoner can be signalled, false otherwise."
  [archer-awake? prisoner-awake?]
  (and (not archer-awake?)
       prisoner-awake?))

(defn can-free-prisoner?
  "Returns true if prisoner can be freed, false otherwise."
  [knight-awake? archer-awake? prisoner-awake? dog-present?]
  (or (and (not knight-awake?)
           (not archer-awake?)
           prisoner-awake?)
      (and (not archer-awake?)
           dog-present?)))

;; BEGIN LOCAL BENCHMARK ADAPTER

(defn benchmark [rounds]
  (loop [i 0
         checksum 0]
    (if (= i rounds)
      checksum
      (let [knight-awake? (= (mod i 2) 0)
            archer-awake? (= (mod i 3) 0)
            prisoner-awake? (= (mod i 5) 0)
            dog-present? (= (mod i 7) 0)
            c1 (if (can-fast-attack? knight-awake?) (inc checksum) checksum)
            c2 (if (can-spy? knight-awake? archer-awake? prisoner-awake?) (inc c1) c1)
            c3 (if (can-signal-prisoner? archer-awake? prisoner-awake?) (inc c2) c2)
            c4 (if (can-free-prisoner?
                     knight-awake?
                     archer-awake?
                     prisoner-awake?
                     dog-present?)
                 (inc c3)
                 c3)]
        (recur (inc i) c4)))))

(defn -main [] (println (benchmark 2000000)))

(-main)
