;; ADR-0013 Gate 3 differential corpus — JVM/Pedestal side.
;;
;; Mirrors ../../native_chain.clj scenario for scenario, but executes each chain
;; with the pinned upstream io.pedestal.interceptor.chain. The context installs
;; the same termination rule the native connector applies (terminate as soon as
;; the context carries a :response), so enter/leave order, termination, unwind,
;; and recovery are compared on equal footing. Exception identity is deliberately
;; not observed — ADR-0013 records that typed Pedestal/JVM exception identity is
;; unavailable in P1 — so :error handlers only recover, matching the native side.
;;
;; The printed "name value" lines must be byte-identical to the native output;
;; run.sh diffs them.
(ns jvm-chain
  (:require [io.pedestal.interceptor.chain :as chain]
            [io.pedestal.interceptor :as interceptor]))

(defn- ic [m] (interceptor/interceptor m))

(defn tracer [name]
  (ic {:name (keyword name)
       :enter (fn [ctx] (assoc ctx :trace (str (get ctx :trace) "e" name)))
       :leave (fn [ctx] (assoc ctx :trace (str (get ctx :trace) "l" name)))}))

(defn responder [name status]
  (ic {:name (keyword name)
       :enter (fn [ctx] (assoc ctx :trace (str (get ctx :trace) name)
                              :response {:status status}))}))

(defn- run [interceptors]
  (-> {:trace ""}
      (chain/terminate-when #(contains? % :response))
      (chain/execute interceptors)))

(defn trace-of [interceptors]
  (get (run interceptors) :trace))

(defn -main [& _]
  (println "order3" (trace-of [(tracer "1") (tracer "2") (tracer "3")]))
  (println "single" (trace-of [(tracer "1")]))
  (let [c (run [(tracer "a") (responder "t" 200) (tracer "b")])]
    (println "terminate-mid" (str (get c :trace) "/" (get (get c :response) :status))))
  (let [c (run [(responder "t" 201) (tracer "a") (tracer "b")])]
    (println "terminate-first" (str (get c :trace) "/" (get (get c :response) :status))))
  (let [boom (ic {:name :boom :enter (fn [_] (throw (ex-info "boom" {:oops 1})))})
        guard (ic {:name :guard :error (fn [ctx _err] (assoc ctx :response {:status 500} :recovered true))})
        c (run [guard boom])]
    (println "recover" (str (get (get c :response) :status) "/" (get c :recovered))))
  (let [c (run [(tracer "x") (tracer "y") (responder "z" 200)])]
    (println "terminate-last" (str (get c :trace) "/" (get (get c :response) :status)))))
