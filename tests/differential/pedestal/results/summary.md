# ADR-0013 Gate 3 differential — interceptor chain

Native cljn.pedestal.chain output equals pinned Pedestal/JVM (0.8.2-beta-10) for every scenario (order, termination, unwind, recovery). Regenerate with `tests/differential/pedestal/run.sh`.

```
order3 e1e2e3l3l2l1
single e1l1
terminate-mid eatla/200
terminate-first t/201
recover 500/true
terminate-last exeyzlylx/200
```
