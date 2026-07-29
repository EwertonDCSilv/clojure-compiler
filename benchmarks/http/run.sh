#!/usr/bin/env bash
# ADR-0013 Gate 6 — HTTP benchmark orchestrator (Native vs Clojure/JVM Pedestal).
#
# Drives the compiled cljn.pedestal.* connector and the pinned upstream Pedestal
# http-kit connector through one identical load client, proves both serve
# byte-identical responses, and records timing over several repetitions. The
# benchmark is versioned separately from the language conformance suite and the
# Cracking/Cormen/Exercism performance catalog (ADR-0013 §12) and touches no
# runtime code, so it cannot affect those results.
#
# It needs a JVM, the Clojure CLI, and network access to Clojars/Maven Central to
# resolve Pedestal, so — like the manual JVM conformance oracle — it runs on
# demand and not in CI. Results are committed under results/.
#
# Usage: benchmarks/http/run.sh [--reps N] [--requests N] [--warmup N] [--out DIR]
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
here="${repo_root}/benchmarks/http"
reps=5
requests=20000
warmup=2000
out_dir="${here}/results"
compiler="${repo_root}/target/release/clojure-native"

while (($# > 0)); do
  case "$1" in
    --reps) reps="$2"; shift 2 ;;
    --requests) requests="$2"; shift 2 ;;
    --warmup) warmup="$2"; shift 2 ;;
    --out) out_dir="$2"; shift 2 ;;
    -h|--help)
      grep '^#' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0 ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
done

# Locate the Clojure CLI, preferring a user-local install.
if command -v clojure >/dev/null 2>&1; then
  clojure_bin="clojure"
elif [ -x "${HOME}/.clojure-cli/bin/clojure" ]; then
  clojure_bin="${HOME}/.clojure-cli/bin/clojure"
else
  printf 'clojure CLI not found; install it to resolve the pinned Pedestal deps\n' >&2
  exit 1
fi
command -v java >/dev/null 2>&1 || { printf 'java not found\n' >&2; exit 1; }

if [ ! -x "$compiler" ]; then
  printf 'building the native compiler...\n' >&2
  (cd "$repo_root" && cargo build --release --locked -p clojure-native-cli >/dev/null)
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"; [ -n "${native_pid:-}" ] && kill "$native_pid" 2>/dev/null || true; [ -n "${jvm_pid:-}" ] && kill "$jvm_pid" 2>/dev/null || true' EXIT

free_port() {
  python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1",0)); print(s.getsockname()[1]); s.close()'
}

wait_ready() {
  local port="$1" tries=0
  while ((tries < 100)); do
    if curl -s -o /dev/null "http://127.0.0.1:${port}/greet"; then return 0; fi
    sleep 0.1; ((tries++))
  done
  return 1
}

# --- Build the native benchmark server ---------------------------------------
printf 'building the native greet server...\n' >&2
native_bin="${work}/greet-native"
"$compiler" build "${here}/native/greet_server.clj" -o "$native_bin" >/dev/null

# --- Resolve the pinned Pedestal classpath (warms the local cache) -----------
printf 'resolving the pinned Pedestal classpath...\n' >&2
(cd "${here}/jvm" && "$clojure_bin" -Spath >/dev/null)

native_results="${work}/native.jsonl"
jvm_results="${work}/jvm.jsonl"
: >"$native_results"
: >"$jvm_results"

run_client() {
  local port="$1"
  "$clojure_bin" -M "${here}/client/load_client.clj" 127.0.0.1 "$port" "$requests" "$warmup"
}

# --- Native repetitions ------------------------------------------------------
for ((r = 1; r <= reps; r++)); do
  printf 'native rep %d/%d\n' "$r" "$reps" >&2
  out="${work}/native-${r}.out"
  ( ulimit -v 4000000; exec timeout 180 "$native_bin" >"$out" 2>&1 ) &
  native_pid=$!
  port=""
  for _ in $(seq 1 100); do port="$(head -1 "$out" 2>/dev/null || true)"; [ -n "$port" ] && break; sleep 0.1; done
  [ -n "$port" ] || { printf 'native server did not report a port\n' >&2; exit 1; }
  wait_ready "$port" || { printf 'native server not ready\n' >&2; exit 1; }
  run_client "$port" >>"$native_results"
  kill "$native_pid" 2>/dev/null || true
  wait "$native_pid" 2>/dev/null || true
  native_pid=""
done

# --- JVM repetitions ---------------------------------------------------------
for ((r = 1; r <= reps; r++)); do
  printf 'jvm rep %d/%d\n' "$r" "$reps" >&2
  port="$(free_port)"
  ( cd "${here}/jvm" && exec "$clojure_bin" -M -m greet-server "$port" ) >"${work}/jvm-${r}.out" 2>&1 &
  jvm_pid=$!
  wait_ready "$port" || { printf 'jvm server not ready\n' >&2; cat "${work}/jvm-${r}.out" >&2; exit 1; }
  run_client "$port" >>"$jvm_results"
  kill "$jvm_pid" 2>/dev/null || true
  wait "$jvm_pid" 2>/dev/null || true
  jvm_pid=""
done

# --- Environment metadata ----------------------------------------------------
mkdir -p "$out_dir"
export BENCH_GIT_REV="$(cd "$repo_root" && git rev-parse --short HEAD 2>/dev/null || echo unknown)"
export BENCH_JAVA_VER="$("$clojure_bin" -e '(print (System/getProperty "java.runtime.version"))' 2>/dev/null || echo unknown)"
export BENCH_CLJ_VER="$("$clojure_bin" -e '(print (clojure-version))' 2>/dev/null || echo unknown)"
export BENCH_CPU="$(grep -m1 'model name' /proc/cpuinfo 2>/dev/null | sed 's/.*: //' || echo unknown)"
export BENCH_CORES="$(nproc 2>/dev/null || echo unknown)"

# --- Aggregate and emit report ----------------------------------------------
python3 - "$native_results" "$jvm_results" "$out_dir" <<'PY'
import json, statistics, sys, datetime, os

native_path, jvm_path, out_dir = sys.argv[1], sys.argv[2], sys.argv[3]

def load(path):
    rows = []
    with open(path) as fh:
        for line in fh:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows

def med(values):
    return round(statistics.median(values), 3)

def agg(rows):
    return {
        "repetitions": len(rows),
        "requests_per_rep": rows[0]["requests"] if rows else 0,
        "throughput_rps_median": med([r["throughput_rps"] for r in rows]),
        "throughput_rps_all": [r["throughput_rps"] for r in rows],
        "mean_us_median": med([r["mean_us"] for r in rows]),
        "p50_us_median": med([r["p50_us"] for r in rows]),
        "p95_us_median": med([r["p95_us"] for r in rows]),
        "p99_us_median": med([r["p99_us"] for r in rows]),
        "max_us_median": med([r["max_us"] for r in rows]),
        "body_sha256": sorted({r["body_sha256"] for r in rows}),
        "mismatches_total": sum(r["mismatches"] for r in rows),
    }

native = load(native_path)
jvm = load(jvm_path)
n_agg = agg(native)
j_agg = agg(jvm)

bodies = set(n_agg["body_sha256"]) | set(j_agg["body_sha256"])
equivalent = (len(bodies) == 1
              and n_agg["mismatches_total"] == 0
              and j_agg["mismatches_total"] == 0)

speedup = (round(n_agg["throughput_rps_median"] / j_agg["throughput_rps_median"], 3)
           if j_agg["throughput_rps_median"] else None)
latency_ratio = (round(j_agg["p50_us_median"] / n_agg["p50_us_median"], 3)
                 if n_agg["p50_us_median"] else None)

report = {
    "benchmark": "adr-0013-http-hello-world",
    "generated": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "route": "GET /greet -> 200 \"Hello, world!\\n\"",
    "load_model": "closed-loop, single connection, fresh socket per request, Connection: close",
    "equivalent_response": equivalent,
    "shared_body_sha256": sorted(bodies),
    "environment": {
        "git_rev": os.environ.get("BENCH_GIT_REV", "unknown"),
        "cpu": os.environ.get("BENCH_CPU", "unknown"),
        "cores": os.environ.get("BENCH_CORES", "unknown"),
        "java_runtime": os.environ.get("BENCH_JAVA_VER", "unknown"),
        "clojure": os.environ.get("BENCH_CLJ_VER", "unknown"),
        "pedestal": "0.8.2-beta-10",
        "http_kit": "2.8.1",
    },
    "native": n_agg,
    "jvm": j_agg,
    "native_over_jvm_throughput": speedup,
    "jvm_over_native_p50_latency": latency_ratio,
}

os.makedirs(out_dir, exist_ok=True)
with open(os.path.join(out_dir, "http-benchmark.json"), "w") as fh:
    json.dump(report, fh, indent=2)
    fh.write("\n")

def line(label, n, j):
    return f"| {label} | {n} | {j} |"

env = report["environment"]
md = []
md.append("# HTTP benchmark — Native vs Clojure/JVM Pedestal")
md.append("")
md.append("ADR-0013 Gate 6. Generated by `benchmarks/http/run.sh`. Both servers expose the")
md.append("same minimal route and serve byte-identical responses; only timing differs.")
md.append("")
md.append(f"- Response equivalence: **{'confirmed' if equivalent else 'FAILED'}** "
          f"(shared body SHA-256 `{(sorted(bodies) or [''])[0][:16]}…`)")
md.append(f"- Repetitions: {n_agg['repetitions']} × {n_agg['requests_per_rep']} requests")
md.append(f"- Route: `GET /greet` → `200 \"Hello, world!\\n\"`")
md.append(f"- Environment: {env['cpu']}, {env['cores']} cores, "
          f"Java {env['java_runtime']}, Clojure {env['clojure']}, "
          f"Pedestal {env['pedestal']} / http-kit {env['http_kit']}")
md.append("")
md.append("| Metric (median of reps) | Native | Clojure/JVM Pedestal |")
md.append("| --- | ---: | ---: |")
md.append(line("Throughput (req/s)", n_agg["throughput_rps_median"], j_agg["throughput_rps_median"]))
md.append(line("Mean latency (µs)", n_agg["mean_us_median"], j_agg["mean_us_median"]))
md.append(line("p50 latency (µs)", n_agg["p50_us_median"], j_agg["p50_us_median"]))
md.append(line("p95 latency (µs)", n_agg["p95_us_median"], j_agg["p95_us_median"]))
md.append(line("p99 latency (µs)", n_agg["p99_us_median"], j_agg["p99_us_median"]))
md.append("")
if speedup is not None:
    md.append(f"Native serves **{speedup}×** the JVM throughput and **{latency_ratio}×** lower "
              f"median latency in this environment.")
md.append("")
md.append("> Engineering measurement, not a universal claim. Compare only within the same")
md.append("> environment, revision, and toolchain (see benchmarks/README.md).")
md.append("")
with open(os.path.join(out_dir, "summary.md"), "w") as fh:
    fh.write("\n".join(md))

print("equivalent_response:", equivalent)
print("native  median rps:", n_agg["throughput_rps_median"], "p50 us:", n_agg["p50_us_median"])
print("jvm     median rps:", j_agg["throughput_rps_median"], "p50 us:", j_agg["p50_us_median"])
if not equivalent:
    sys.exit(3)
PY

printf 'wrote %s/http-benchmark.json and %s/summary.md\n' "$out_dir" "$out_dir" >&2
