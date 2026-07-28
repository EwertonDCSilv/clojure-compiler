# Upstream attribution and provenance

The original reference implementations used by this benchmark corpus come from the
[Exercism Clojure Track](https://github.com/exercism/clojure), maintained by the
Exercism community.

## Pinned source

- Official repository: [`exercism/clojure`](https://github.com/exercism/clojure)
- Exercism track: [Clojure on Exercism](https://exercism.org/tracks/clojure)
- Audited snapshot:
  [`4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190`](https://github.com/exercism/clojure/tree/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190)
- Upstream license:
  [MIT](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/LICENSE)
- Local license copy: [`LICENSE.exercism`](LICENSE.exercism)

Copyright in the original reference implementations remains with Exercism and its
contributors under the MIT License.

## Local changes

The eight performance fixtures preserve the upstream implementation before an explicit
local-adapter marker. This repository adds deterministic inputs, a `benchmark`
function, a `-main` checksum entry point, runners, reports and performance analysis.

Separately, all 13 official concept exemplars are versioned as cases in the
[Exercism conformance corpus](../../tests/conformance/level-d-pure-libraries/external/exercism/).
Its audit checks each exemplar body against the pinned checkout. The complete 493-file
checkout is audited externally and is not copied into this repository.

This suite is an independent compiler-engineering corpus. It is not an official
Exercism benchmark, and the results do not imply endorsement by or affiliation with
Exercism.

## Performance fixture-to-source map

| Local fixture | Original source at the pinned commit |
| --- | --- |
| [`01-accumulate.clj`](01-practice/01-accumulate.clj) | [`accumulate/.meta/example.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/accumulate/.meta/example.clj) |
| [`02-binary-search.clj`](01-practice/02-binary-search.clj) | [`binary-search/.meta/example.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/binary-search/.meta/example.clj) |
| [`03-hello-world.clj`](01-practice/03-hello-world.clj) | [`hello-world/.meta/example.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/hello-world/.meta/example.clj) |
| [`04-knapsack.clj`](01-practice/04-knapsack.clj) | [`knapsack/.meta/example.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/knapsack/.meta/example.clj) |
| [`05-prime-factors.clj`](01-practice/05-prime-factors.clj) | [`prime-factors/.meta/example.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/prime-factors/.meta/example.clj) |
| [`06-square-root.clj`](01-practice/06-square-root.clj) | [`square-root/.meta/example.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/square-root/.meta/example.clj) |
| [`07-two-fer.clj`](01-practice/07-two-fer.clj) | [`two-fer/.meta/example.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/two-fer/.meta/example.clj) |
| [`01-annalyns-infiltration.clj`](02-concept/01-annalyns-infiltration.clj) | [`annalyns-infiltration/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/annalyns-infiltration/.meta/exemplar.clj) |

## Conformance case-to-source map

| Conformance fixture | Original exemplar at the pinned commit |
| --- | --- |
| [`annalyns-infiltration/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/annalyns-infiltration/input.clj) | [`annalyns-infiltration/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/annalyns-infiltration/.meta/exemplar.clj) |
| [`bird-watcher/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/bird-watcher/input.clj) | [`bird-watcher/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/bird-watcher/.meta/exemplar.clj) |
| [`card-games/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/card-games/input.clj) | [`card-games/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/card-games/.meta/exemplar.clj) |
| [`cars-assemble/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/cars-assemble/input.clj) | [`cars-assemble/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/cars-assemble/.meta/exemplar.clj) |
| [`coordinate-transformation/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/coordinate-transformation/input.clj) | [`coordinate-transformation/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/coordinate-transformation/.meta/exemplar.clj) |
| [`date-parser/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/date-parser/input.clj) | [`date-parser/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/date-parser/.meta/exemplar.clj) |
| [`elyses-destructured-enchantments/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/elyses-destructured-enchantments/input.clj) | [`elyses-destructured-enchantments/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/elyses-destructured-enchantments/.meta/exemplar.clj) |
| [`interest-is-interesting/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/interest-is-interesting/input.clj) | [`interest-is-interesting/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/interest-is-interesting/.meta/exemplar.clj) |
| [`international-calling-connoisseur/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/international-calling-connoisseur/input.clj) | [`international-calling-connoisseur/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/international-calling-connoisseur/.meta/exemplar.clj) |
| [`log-levels/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/log-levels/input.clj) | [`log-levels/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/log-levels/.meta/exemplar.clj) |
| [`lucians-luscious-lasagna/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/lucians-luscious-lasagna/input.clj) | [`lucians-luscious-lasagna/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/lucians-luscious-lasagna/.meta/exemplar.clj) |
| [`squeaky-clean/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/squeaky-clean/input.clj) | [`squeaky-clean/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/squeaky-clean/.meta/exemplar.clj) |
| [`tracks-on-tracks-on-tracks/input.clj`](../../tests/conformance/level-d-pure-libraries/external/exercism/tracks-on-tracks-on-tracks/input.clj) | [`tracks-on-tracks-on-tracks/.meta/exemplar.clj`](https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/concept/tracks-on-tracks-on-tracks/.meta/exemplar.clj) |
