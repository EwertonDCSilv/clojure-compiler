;; Shared HTTP load client for the ADR-0013 Gate 6 benchmark.
;;
;; One identical client drives both servers so the measurement, not the tool,
;; is what differs. It runs a closed-loop, single-connection sequential load:
;; for each request it opens a fresh TCP socket, sends a GET /greet request with
;; `Connection: close`, reads the whole response to EOF, and records the wall
;; latency around that exchange. `Connection: close` makes both the native
;; provider and http-kit close the socket after the response, so the same
;; read-to-EOF logic works for either server.
;;
;; Every response is validated: the status line must be `HTTP/1.1 200 OK` and the
;; body must equal the expected greeting. The body SHA-256 is emitted so the
;; orchestrator can prove both servers serve byte-identical content before it
;; compares any timing. Results are printed as one JSON object on stdout.
;;
;; Usage: clojure -M load_client.clj HOST PORT REQUESTS WARMUP
(ns load-client
  (:import [java.net Socket InetSocketAddress]
           [java.io ByteArrayOutputStream]
           [java.security MessageDigest]
           [java.util Locale]))

(def ^:private expected-body "Hello, world!\n")
(def ^:private request-bytes
  (.getBytes "GET /greet HTTP/1.1\r\nHost: bench\r\nConnection: close\r\n\r\n" "US-ASCII"))

(defn- read-fully
  "Read the socket input stream to EOF and return the raw response bytes."
  [in]
  (let [buffer (byte-array 8192)
        out (ByteArrayOutputStream.)]
    (loop []
      (let [n (.read in buffer)]
        (when (pos? n)
          (.write out buffer 0 n)
          (recur))))
    (.toByteArray out)))

(defn- one-request
  "Perform a single request against host:port; return [latency-nanos ^bytes response]."
  [host port]
  (let [socket (Socket.)]
    (try
      (.setTcpNoDelay socket true)
      (let [start (System/nanoTime)]
        (.connect socket (InetSocketAddress. host (int port)) 2000)
        (doto (.getOutputStream socket)
          (.write request-bytes)
          (.flush))
        (let [response (read-fully (.getInputStream socket))]
          [(- (System/nanoTime) start) response]))
      (finally
        (.close socket)))))

(defn- response-body
  "Split an HTTP/1.x response into [status-line body-string]."
  [^bytes response]
  (let [text (String. response "UTF-8")
        idx (.indexOf text "\r\n\r\n")
        status (subs text 0 (max 0 (.indexOf text "\r\n")))
        body (if (neg? idx) "" (subs text (+ idx 4)))]
    [status body]))

(defn- sha256-hex [^String s]
  (let [digest (.digest (MessageDigest/getInstance "SHA-256") (.getBytes s "UTF-8"))]
    (apply str (map #(format "%02x" %) digest))))

(defn- percentile [sorted p]
  (let [n (count sorted)]
    (if (zero? n)
      0
      (nth sorted (min (dec n) (int (Math/floor (* p (dec n)))))))))

(defn -main [& args]
  (let [[host port-s requests-s warmup-s] args
        port (Integer/parseInt port-s)
        requests (Integer/parseInt requests-s)
        warmup (Integer/parseInt (or warmup-s "0"))]
    (dotimes [_ warmup] (one-request host port))
    (let [latencies (long-array requests)
          started (System/nanoTime)]
      (loop [i 0 mismatches 0 body-hash nil]
        (if (< i requests)
          (let [[nanos response] (one-request host port)
                [status body] (response-body response)
                ok? (and (= status "HTTP/1.1 200 OK") (= body expected-body))]
            (aset latencies i nanos)
            (recur (inc i)
                   (if ok? mismatches (inc mismatches))
                   (or body-hash (sha256-hex body))))
          (let [duration-ns (- (System/nanoTime) started)
                sorted (vec (sort (seq latencies)))
                us (fn [ns] (/ (Math/round (/ (double ns) 10.0)) 100.0))
                total-ms (/ (Math/round (/ (double duration-ns) 1000.0)) 1000.0)
                mean-ns (/ (double (reduce + (seq latencies))) requests)
                rps (Math/round (/ (double requests) (/ (double duration-ns) 1.0e9)))]
            (println
              (String/format
                Locale/US
                (str "{\"requests\":%d,\"duration_ms\":%.3f,\"throughput_rps\":%d,"
                     "\"mean_us\":%.2f,\"p50_us\":%.2f,\"p95_us\":%.2f,\"p99_us\":%.2f,"
                     "\"max_us\":%.2f,\"body_sha256\":\"%s\",\"mismatches\":%d}")
                (to-array
                  [(int requests) (double total-ms) (long rps)
                   (double (us mean-ns))
                   (double (us (percentile sorted 0.50)))
                   (double (us (percentile sorted 0.95)))
                   (double (us (percentile sorted 0.99)))
                   (double (us (percentile sorted 1.0)))
                   (or body-hash "") (int mismatches)])))))))))

(apply -main *command-line-args*)
