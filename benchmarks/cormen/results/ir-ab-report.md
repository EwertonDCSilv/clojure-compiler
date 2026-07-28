# Cormen native IR A/B

- Control: `--ir-opt none --opt-level none`
- Candidate: `--ir-opt safe --opt-level none`
- Repetitions: 7 paired and alternating
- Scale: 25
- Cases: 30
- Gate: **PASS**

| Scope | Wall candidate/control | Wall MAD | Wall 95% CI | CPU candidate/control | CPU MAD | CPU 95% CI |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Aggregate | 0.9568 | 0.0079 | 0.9547–0.9799 | 0.9565 | 0.0070 | 0.9550–0.9814 |
| 01-foundations-and-divide-conquer | 0.9227 | 0.0051 | 0.9176–0.9282 | 0.9274 | 0.0052 | 0.9218–0.9278 |
| 02-sorting-and-order-statistics | 0.9692 | 0.0128 | 0.9564–0.9888 | 0.9717 | 0.0170 | 0.9507–0.9887 |
| 03-data-structures | 0.9396 | 0.0119 | 0.9277–0.9524 | 0.9406 | 0.0158 | 0.9248–0.9569 |
| 04-dynamic-programming-and-greedy | 0.9557 | 0.0131 | 0.9361–0.9637 | 0.9533 | 0.0102 | 0.9381–0.9636 |
| 05-graph-algorithms | 0.9824 | 0.0214 | 0.9610–1.0046 | 0.9823 | 0.0247 | 0.9584–1.0070 |
| 06-number-theory-and-string-matching | 0.9797 | 0.0087 | 0.9710–1.0027 | 0.9798 | 0.0101 | 0.9697–1.0053 |

## Cases

| Benchmark | Wall candidate/control | Wall 95% CI | CPU candidate/control | CPU 95% CI | RSS none KiB | RSS safe KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `01-foundations-and-divide-conquer/01-binary-exponentiation.clj` | 0.4286 | 0.4286–0.4286 | 0.4286 | 0.3333–0.5000 | 1480 | 1480 |
| `01-foundations-and-divide-conquer/02-horner-polynomial.clj` | 0.8333 | 0.8333–0.8333 | 0.8333 | 0.8333–0.8333 | 1480 | 1480 |
| `01-foundations-and-divide-conquer/03-prefix-range-sums.clj` | 0.8750 | 0.8750–1.0000 | 0.8750 | 0.8571–1.0000 | 11976 | 12120 |
| `01-foundations-and-divide-conquer/04-iterative-binary-search.clj` | 0.8333 | 0.8235–0.8824 | 0.8333 | 0.8235–0.8824 | 1480 | 1480 |
| `01-foundations-and-divide-conquer/05-maximum-subarray-divide.clj` | 0.9648 | 0.9510–0.9716 | 0.9648 | 0.9577–0.9786 | 5320 | 5320 |
| `02-sorting-and-order-statistics/01-insertion-sort.clj` | 0.9474 | 0.9180–1.0182 | 0.9643 | 0.9016–1.0185 | 18760 | 18632 |
| `02-sorting-and-order-statistics/02-selection-sort.clj` | 0.9167 | 0.8919–0.9444 | 0.9167 | 0.8889–0.9429 | 18760 | 18632 |
| `02-sorting-and-order-statistics/03-counting-sort.clj` | 0.9565 | 0.9130–1.0000 | 0.9130 | 0.9091–0.9565 | 14920 | 14920 |
| `02-sorting-and-order-statistics/04-merge-sort.clj` | 0.9292 | 0.9083–0.9815 | 0.9286 | 0.9074–0.9813 | 16456 | 16456 |
| `02-sorting-and-order-statistics/05-quickselect.clj` | 0.9927 | 0.9853–1.0150 | 1.0000 | 0.9926–1.0149 | 4808 | 4808 |
| `03-data-structures/01-build-max-heap.clj` | 0.8788 | 0.8750–0.9677 | 0.8710 | 0.8438–1.0000 | 18632 | 18760 |
| `03-data-structures/02-disjoint-set-union.clj` | 0.9753 | 0.9625–1.0000 | 0.9750 | 0.9625–1.0000 | 13640 | 13768 |
| `03-data-structures/03-chained-hash-table.clj` | 0.9636 | 0.9474–1.0185 | 0.9815 | 0.9455–1.0185 | 13252 | 13256 |
| `03-data-structures/04-circular-queue.clj` | 0.6957 | 0.6818–0.7273 | 0.6818 | 0.6818–0.6818 | 13384 | 13384 |
| `03-data-structures/05-binary-search-tree.clj` | 0.9641 | 0.9155–1.0000 | 0.9685 | 0.9198–1.0048 | 19884 | 19912 |
| `04-dynamic-programming-and-greedy/01-rod-cutting.clj` | 0.9583 | 0.9531–0.9681 | 0.9626 | 0.9531–0.9681 | 7624 | 7624 |
| `04-dynamic-programming-and-greedy/02-matrix-chain-order.clj` | 0.9237 | 0.8978–0.9394 | 0.9231 | 0.9044–0.9328 | 13000 | 12996 |
| `04-dynamic-programming-and-greedy/03-longest-common-subsequence.clj` | 0.9537 | 0.9074–0.9972 | 0.9536 | 0.9096–0.9945 | 15176 | 15304 |
| `04-dynamic-programming-and-greedy/04-zero-one-knapsack.clj` | 0.9663 | 0.9660–0.9854 | 0.9659 | 0.9615–0.9854 | 5700 | 5704 |
| `04-dynamic-programming-and-greedy/05-activity-selection.clj` | 0.8000 | 0.8000–0.8000 | 0.8000 | 0.8000–0.8000 | 1496 | 1480 |
| `05-graph-algorithms/01-breadth-first-search.clj` | 0.9444 | 0.8972–1.0300 | 0.9352 | 0.8962–1.0404 | 19784 | 19784 |
| `05-graph-algorithms/02-depth-first-search.clj` | 1.0116 | 0.9560–1.1047 | 1.0118 | 0.9451–1.1059 | 20040 | 20040 |
| `05-graph-algorithms/03-topological-sort.clj` | 1.0000 | 0.9466–1.0370 | 1.0079 | 0.9466–1.0435 | 19208 | 19272 |
| `05-graph-algorithms/04-bellman-ford.clj` | 0.9750 | 0.9512–1.0000 | 0.9756 | 0.9500–1.0000 | 15304 | 15304 |
| `05-graph-algorithms/05-floyd-warshall.clj` | 0.9706 | 0.9296–0.9855 | 0.9565 | 0.9296–0.9701 | 15176 | 15176 |
| `06-number-theory-and-string-matching/01-extended-euclid.clj` | 0.9907 | 0.9269–1.0893 | 0.9860 | 0.9231–1.0852 | 19912 | 19912 |
| `06-number-theory-and-string-matching/02-sieve-of-eratosthenes.clj` | 1.0228 | 0.9720–1.0717 | 1.0228 | 0.9696–1.0697 | 21832 | 21832 |
| `06-number-theory-and-string-matching/03-naive-string-matching.clj` | 0.7826 | 0.7660–0.7826 | 0.7826 | 0.7660–0.7826 | 1480 | 1480 |
| `06-number-theory-and-string-matching/04-rabin-karp.clj` | 0.7407 | 0.7308–0.7407 | 0.7407 | 0.7037–0.7407 | 1480 | 1480 |
| `06-number-theory-and-string-matching/05-knuth-morris-pratt.clj` | 0.8421 | 0.8421–0.9444 | 0.8889 | 0.8889–0.8889 | 11976 | 11976 |
