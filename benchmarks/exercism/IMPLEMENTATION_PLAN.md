# Plan to promote the Exercism compatibility corpus

This plan turns 101 public practice reference solutions and 13 official concept
exemplars into an evolving external gate for `clojure-compiler`. It is based on upstream commit
[`4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190`](https://github.com/exercism/clojure/tree/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190)
from the [Exercism Clojure Track](https://github.com/exercism/clojure), with full
provenance documented in [`UPSTREAM.md`](UPSTREAM.md), and was most recently audited
with compiler
[`424ba20`](https://github.com/EwertonDCSilv/clojure-compiler/commit/424ba20e88fd91a641675e4d9d9bf111c63fc164).

## Baseline

- 101 reference implementations audited.
- 13 concept exemplars versioned and checked against upstream before compilation.
- 493 Clojure files compiled individually across the complete checkout.
- 10 of the 114 complete official solutions build without changes.
- 104 stop at a compiler diagnostic.
- Across all roles, 117 files build and 376 stop at a diagnostic.
- 13 concept exemplars are executable conformance fixtures: 2 active and 11 xfail;
  3 compile in the looser inventory.
- 8 useful workloads have separate Native × JVM benchmark adapters.
- Java interop remains outside the current native language contract.

The counts below are first blockers, not independent feature totals:

| First blocker | Cases | Intended implementation track |
| --- | ---: | --- |
| top-level collection/computed `def` | 20 | analyzer, global initialization and permanent GC roots |
| missing core function | 16 | compiled `clojure.core` and native primitives |
| regex literal/runtime | 16 | reader value, regex engine and string APIs |
| missing core macro | 11 | macro expansion before native analysis |
| destructuring | 5 | parameter, `let` and `loop` lowering |
| missing arity | 4 | call validation and stdlib arities |
| Java interop operation | 5 | explicit compatibility decision |
| variadic primitive used as a value | 3 | first-class primitive wrapper |
| unresolved core symbol/predicate | 3 | core inventory and function values |
| quote | 2 | quoted value lowering and rooting |
| syntax quote | 2 | analyzer macro infrastructure |
| standard-library namespace | 2 | `clojure.string` and `clojure.math` slices |
| exception constructor | 2 | native exception construction |
| symbol ending in apostrophe | 2 | reader tokenization |
| nested/multi-arity `fn` form | 1 | function analyzer |
| **Total failing** | **94** | |

The authoritative per-exercise list is
[`results/compilation.tsv`](results/compilation.tsv).

The practice table above remains a 101-solution baseline. The separate concept
conformance matrix is
[`compilation.tsv`](../../tests/conformance/level-d-pure-libraries/external/exercism/compilation.tsv):

| Concept first blocker | Cases |
| --- | ---: |
| top-level computed `def` | 5 |
| regex literal/runtime | 2 |
| missing core function (`boolean`, `atom`) | 2 |
| parameter destructuring | 1 |
| BigDecimal | 1 |
| `quote` | 1 |
| supported and promoted | 1 |
| **Total** | **13** |

## Backlog by first blocker

- `top-level-def` (20): `allergies`, `diamond`, `food-chain`, `hexadecimal`,
  `high-scores`, `killer-sudoku-helper`, `kindergarten-garden`, `meetup`,
  `nucleotide-count`, `proverb`, `queen-attack`, `raindrops`, `resistor-color-duo`,
  `resistor-color-trio`, `resistor-color`, `rna-transcription`, `robot-simulator`,
  `roman-numerals`, `scrabble-score`, `space-age`.
- `missing-core-function` (16): `anagram`, `bank-account`, `beer-song`, `clock`,
  `collatz-conjecture`, `dnd-character`, `eliuds-eggs`, `isogram`, `list-ops`,
  `perfect-numbers`, `pov`, `state-of-tic-tac-toe`, `strain`, `sum-of-multiples`,
  `yacht`, `zipper`.
- `reader-regex` (16): `acronym`, `atbash-cipher`, `binary`, `connect`,
  `crypto-square`, `isbn-verifier`, `matrix`, `phone-number`, `pig-latin`, `poker`,
  `robot-name`, `run-length-encoding`, `say`, `twelve-days`, `word-count`, `wordy`.
- `core-macro` (11): `darts`, `dominoes`, `etl`, `game-of-life`, `gigasecond`,
  `go-counting`, `grade-school`, `protein-translation`, `pythagorean-triplet`,
  `sieve`, `transpose`.
- `destructuring` (5): `all-your-base`, `complex-numbers`, `leap`,
  `matching-brackets`, `spiral-matrix`.
- `missing-arity` (4): `binary-search-tree`, `difference-of-squares`, `pangram`,
  `triangle`.
- `java-interop` (5): `luhn`, `octal`, `rotational-cipher`, `secret-handshake`,
  `trinary`.
- `variadic-primitive-value` (3): `reverse-string`, `saddle-points`,
  `simple-cipher`.
- `missing-symbol` (3): `flatten-array`, `hamming`, `sublist`.
- `quote` (2): `armstrong-numbers`, `grains`.
- `syntax-quote` (2): `flower-field`, `minesweeper`.
- `stdlib-namespace` (2): `bob`, `nth-prime`.
- `exception-constructor` (2): `largest-series-product`, `series`.
- `symbol-apostrophe` (2): `pascals-triangle`, `zebra-puzzle`.
- `fn-form` (1): `change`.

## Phase 0 — Keep the corpus trustworthy

1. Pin the upstream commit in every published report.
2. Run the `references`, `concepts` and `all` inventories in a non-blocking scheduled
   job and upload their TSVs.
3. Add a comparison command that reports status transitions against the versioned TSV.
4. Fail when a former `PASS` becomes `FAIL`.
5. Treat a new `PASS` as requiring semantic review, not automatic promotion.
6. Update the upstream snapshot intentionally; never follow `main` silently.

Acceptance:

- all 101 practice references, 13 versioned concept exemplars and 493 checkout files
  are present exactly once in their respective inventories;
- every concept body matches its pinned upstream exemplar;
- the report records compiler and upstream revisions;
- status transitions are deterministic on Linux x86_64.

## Phase 1 — Analyzer and immutable globals

Implement top-level immutable `def` initialization for strings, vectors, maps, sets,
function values and computed expressions. Values that survive for the whole program
must be registered as permanent roots without reconstructing them at every call.

In parallel, finish:

- destructuring in function parameters, `let` and `loop`;
- the nested/multi-arity `fn` shape used by `change`;
- symbols whose legal name ends with `'`;
- quoted empty and non-empty collections;
- character values through analyzer, IR, ABI and printer.

Primary first-blocker set: 28 cases.

Required tests:

- analyzer unit tests for each binding form;
- global initialization order and duplicate-definition diagnostics;
- GC-stress tests proving global values remain reachable;
- reader round trips for `+'`, `colors'` and characters;
- native/JVM equality for every newly compiling exercise.

## Phase 2 — Core macros and call shapes

Expand supported source forms before code generation:

- `for`, including `:when`;
- `condp`;
- `if-let`;
- `letfn`;
- the specific syntax-quote/unquote paths required by user macros.

Complete call behavior for:

- two-argument `reduce`;
- three-or-more-argument `concat`;
- three-argument `into` only after the transducer contract exists;
- chained numeric comparisons;
- variadic primitives such as `str` and `vector` when passed as values.

Primary first-blocker set: 20 cases.

Acceptance:

- expansion output is covered by analyzer snapshots;
- invalid bindings and arities retain deterministic diagnostics;
- promoted programs pass with `CLJN_GC_STRESS=1`.

## Phase 3 — Fill the compiled core slice

Prioritize small, semantically clear operations before larger subsystems:

1. `not=`, `fnext`, `complement`, `sequential?`, `true?` and `val`;
2. `rem`, `int` and the agreed fixnum behavior of `/`;
3. `frequencies`;
4. `declare`;
5. `atom` only with its documented single-threaded/native semantics;
6. `format` after its supported formatting subset is specified.

`lower-case` belongs to the `clojure.string` phase even when an upstream file refers to
it without an alias.

Primary first-blocker set: 21 cases.

Every function needs normal, boundary, invalid-type and invalid-arity conformance
coverage before an Exercism promotion.

## Phase 4 — Regex and standard-library namespaces

Regex is the largest independent first blocker. Specify and implement:

- compiled regex values from `#"..."`;
- matching, searching, replacement and splitting behavior required by the corpus;
- UTF-8 behavior and unsupported JVM regex features;
- GC ownership for compiled patterns;
- bounded diagnostics for invalid patterns.

Then provide the required portions of:

- `clojure.string`, starting with case conversion, blank checks, split and replace;
- `clojure.math/sqrt`, backed by a documented native floating-point path.

Primary first-blocker set: 18 cases, with more expected after earlier phases expose
secondary regex and string dependencies.

## Phase 5 — Exceptions and the Java boundary

Add native construction for the exception behavior exercised by
`largest-series-product` and `series`.

The four Java-dependent exercises require an architectural decision before code:

- `Character/digit`;
- `Integer/toBinaryString`;
- `.compareTo` in `octal` and `trinary`.

Recommended decision: recognize only a documented compatibility allowlist and lower
these operations to native primitives. Do not introduce general JVM interop into the
runtime. Record the allowlist and semantic differences in an ADR. If that decision is
rejected, keep these four as explicit `expected-diff` cases and provide native-port
fixtures separately.

## Phase 6 — Promotion and benchmark growth

For each status transition:

1. verify the unmodified upstream source builds;
2. execute at least normal, boundary and error inputs on JVM and Native;
3. compare stdout, stderr and exit status;
4. run GC stress where allocation occurs;
5. retain a conformance case even if the workload is unsuitable for benchmarking;
6. add a benchmark adapter only when it measures a distinct runtime/compiler pressure;
7. record checksum, scale, environment and both revisions.

Milestones:

| Milestone | Gate |
| --- | --- |
| E1 | 25/101 upstream sources build, all promoted cases semantically checked |
| E2 | 50/101 build, no regression among prior passes |
| E3 | 75/101 build, regex and stdlib slices active |
| E4 | 97/101 build without Java compatibility |
| E5 | 101/101 build, or 97 plus four approved `expected-diff` decisions |
| C1 | 5/13 concept exemplars build and pass differential checks |
| C2 | 10/13 build with globals, destructuring and core gaps closed |
| C3 | 13/13 build, with regex and BigDecimal decisions documented |

The final gate is not “the compiler accepted the file.” It is matching observable
behavior for exercised inputs and a stable regression test for every feature unlocked.
