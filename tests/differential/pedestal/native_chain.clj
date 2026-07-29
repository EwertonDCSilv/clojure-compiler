;; ADR-0013 Gate 3 differential corpus — native side.
;;
;; Runs a fixed corpus of interceptor-chain scenarios through the compiled
;; cljn.pedestal.chain/execute and prints one normalized "name value" line per
;; scenario. The JVM side (jvm/src/jvm_chain.clj) prints the same lines from the
;; upstream Pedestal io.pedestal.interceptor.chain configured with the same
;; terminate-on-:response rule the native connector applies. run.sh diffs the two
;; outputs and fails on any divergence, so this file is the native observable of
;; the shared interceptor order/termination/unwind/recovery contract
;; (ADR-0013 acceptance #4).
(ns native-chain
  (:require [cljn.pedestal.chain :as chain]))

(defn tracer
  "Interceptor that appends e<name>/l<name> to :trace on enter/leave."
  [name]
  {:name name
   :enter (fn [ctx] (assoc ctx :trace (str (get ctx :trace) "e" name)))
   :leave (fn [ctx] (assoc ctx :trace (str (get ctx :trace) "l" name)))})

(defn responder
  "Interceptor that records itself and sets :response, terminating the enter phase."
  [name status]
  {:name name
   :enter (fn [ctx] (assoc ctx :trace (str (get ctx :trace) name)
                          :response {:status status}))})

(defn trace-of [interceptors]
  (get (chain/execute {:trace ""} interceptors) :trace))

(defn -main []
  ;; enter forward, leave reverse
  (println "order3" (trace-of [(tracer "1") (tracer "2") (tracer "3")]))
  (println "single" (trace-of [(tracer "1")]))
  ;; termination in the middle: later enters skipped, leaves run in reverse
  (let [c (chain/execute {:trace ""} [(tracer "a") (responder "t" 200) (tracer "b")])]
    (println "terminate-mid" (str (get c :trace) "/" (get (get c :response) :status))))
  ;; termination at the first interceptor
  (let [c (chain/execute {:trace ""} [(responder "t" 201) (tracer "a") (tracer "b")])]
    (println "terminate-first" (str (get c :trace) "/" (get (get c :response) :status))))
  ;; error unwind and recovery: a guard's :error sets a response
  (let [boom {:name "boom" :enter (fn [_] (throw {:oops 1}))}
        guard {:name "guard" :error (fn [ctx _err] (assoc ctx :response {:status 500} :recovered true))}
        c (chain/execute {:trace ""} [guard boom])]
    (println "recover" (str (get (get c :response) :status) "/" (get c :recovered))))
  ;; leave-phase mutation ordering with a passthrough interceptor
  (let [c (chain/execute {:trace ""} [(tracer "x") (tracer "y") (responder "z" 200)])]
    (println "terminate-last" (str (get c :trace) "/" (get (get c :response) :status)))))

(-main)
