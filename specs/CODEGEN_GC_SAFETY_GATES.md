# Gates de segurança para mudanças de rooting/GC no codegen

[Índice das especificações](README.md) ·
[Estratégia de testes](TESTING_STRATEGY.md) ·
[ADR-0018](adr/0018-shadow-stack-codegen-micro-optimizations.md)

## Caso de referência

Issue #181 (D1, ADR-0018) introduziu um cache de `gc_sp` (o ponteiro do shadow-stack de
GC) num `Variable` do Cranelift, para evitar um load/store no global a cada
`gc_push_val`/`gc_popn`. A inicialização do cache (`ensure_sp_var`) era **preguiçosa**:
rodava no primeiro `read_sp`/`write_sp` encontrado durante a emissão do corpo da
função — o que, para um corpo com a forma comum
`(if p base (f (dec n) (cons n acc)))`, podia cair dentro de **um único ramo** do `if`.

A instrução Cranelift que carrega `gc_sp` do global só existia nesse ramo. Um
`flush_sp()` alcançável a partir do ramo irmão (ou depois do merge) lia um valor
indefinido/obsoleto ali e o escrevia de volta no global, dessincronizando-o da
profundidade real do shadow-stack. Sob `CLJN_GC_STRESS=1` (coleta a cada alocação),
isso deixava o coletor liberar raízes das quais o programa ainda dependia,
corrompendo o heap — uma lista corrompida virava um ciclo, e o programa entrava em
loop infinito consumindo CPU e memória sem limite.

O bug foi:

- **invisível** para `make pre-commit`, `make compatibility` (385 casos) e
  `make test-runtime-sanitize` (o harness do runtime C testa o runtime isoladamente,
  não a interação do codegen Rust com ele);
- **detectável** apenas por `cargo test -p clojure-native-cli`, especificamente
  `gc_correctness_under_stress` e `native_connector_bodies_and_gc_stress` — os únicos
  testes que compilam e **executam** um programa real sob `CLJN_GC_STRESS=1`;
- capaz de **travar a máquina de desenvolvimento** quando executado sem
  `ulimit -v`/`timeout`, pois o loop infinito aloca sem parar.

O commit que introduziu o bug (`ac41991`) alegava um teste TDD Red→Green
(`adr18_d1_gc_sp_flushed_once_per_call_not_per_push`), mas esse teste só conta
`stats.gc_sp_global_stores` — um proxy estrutural que não executa o binário gerado e,
por isso, não podia detectar a dessincronização.

## Padrão de bug generalizável

Qualquer estado **cacheado em um `cranelift_frontend::Variable`** cuja inicialização
seja condicional ao caminho de código (em vez de garantida no bloco de entrada da
função) está sujeito à mesma classe de bug: o valor inicial não **domina** todos os
usos posteriores, e o Cranelift resolve o uso não dominado para um valor
indefinido/obsoleto sem erro em tempo de compilação. Isso vale para qualquer cache
futuro de estado visível ao runtime C (roots, contadores, ponteiros de frame), não só
`gc_sp`.

## Lacunas nos gates atuais

1. `make test-runtime-sanitize` roda apenas o harness C (`tests/c/*.c`); não compila
   nem executa nenhum programa Clojure. Uma mudança de codegen que afete rooting pode
   passar nesse gate mesmo estando quebrada.
2. `AGENTS.md` recomenda `make test-runtime-sanitize` "para mudanças de GC" mas não
   menciona explicitamente que a suíte E2E de `clojure-native-cli`
   (`cargo test -p clojure-native-cli`) é o gate que realmente exercita o codegen sob
   GC real — é fácil interpretar a recomendação como satisfeita sem rodá-la.
3. Testes estruturais como `adr18_d1_gc_sp_flushed_once_per_call_not_per_push` medem
   uma métrica agregada (contagem de flushes); não verificam a propriedade que
   realmente importa — que o valor lido em cada `use_var` é dominado por uma
   definição correta.
4. Rodar um binário compilado sob `CLJN_GC_STRESS=1` sem `ulimit -v`/`timeout` pode
   travar a máquina inteira se o binário tiver um bug de rooting; não há um wrapper
   padrão no repositório para isso.
5. Não existe uma suíte "smoke" rápida e isolada dedicada a mudanças de rooting/GC —
   a suíte completa (`cargo test -p clojure-native-cli`, ~40-60s) é rápida o bastante
   para rodar sempre, mas nada no fluxo de trabalho **exige** rodá-la antes de commitar
   uma mudança em `enter_planned_frame`/`gc_sp`/`flush_sp`/`sync_sp`/`gc_push_val`/
   `gc_popn`/`leave_frame`.

## Plano proposto

### P1 — Gate obrigatório e explícito para mudanças de rooting/GC (imediato)

Adicionar a `AGENTS.md`, seção Validação, uma linha explícita:

> Para mudanças em `enter_planned_frame`, `gc_sp`/`flush_sp`/`sync_sp`, `gc_push_val`,
> `gc_popn`, `leave_frame`, `gc_frame.rs`, ou qualquer cache de estado visível ao
> runtime C num `cranelift_frontend::Variable`: rode
> `cargo test -p clojure-native-cli` (não apenas `make test-runtime-sanitize`) e
> confirme que `gc_correctness_under_stress` e `native_connector_bodies_and_gc_stress`
> passam. Rode binários compilados manualmente sob `CLJN_GC_STRESS=1` sempre com
> `ulimit -v <limite> timeout <segundos>` — um bug de rooting nesse modo aloca sem
> parar.

Critério de aceite: nenhuma mudança futura em código de rooting é revisável sem essa
evidência explícita no PR (mesma convenção já usada para o gate Cormen A/B nas PRs de
tidy).

### P2 — Teste de caracterização por forma de controle de fluxo (curto prazo)

O teste estrutural atual (`adr18_d1_gc_sp_flushed_once_per_call_not_per_push`) usa um
corpo de função **linear** (sem `if`). Adicionar um segundo teste de caracterização
com exatamente a forma que expôs o bug — chamada recursiva dentro de um só ramo de
`if`, o outro ramo sem nenhuma alocação — e comparar sua saída real sob
`CLJN_GC_STRESS=1` (via `cargo-native-cli` E2E, não só stats) contra o interpretador
de bootstrap como oracle. Esse teste teria pego o bug de #181 diretamente.

Local: `src/compiler/clojure-native-cli/tests/e2e.rs`, junto de
`gc_correctness_under_stress`.

```
(defn conta [n acc]
  (if (< n 0) acc (conta (dec n) (cons n acc))))
```

Critério de aceite: teste falha (hang detectado por timeout, ou saída incorreta) na
revisão do commit `ac41991` sem o fix, e passa com o fix.

### P3 — Helper de execução segura para binários sob GC_STRESS (curto prazo)

Adicionar `scripts/run-gc-stress.sh <binário> [args...]`, um wrapper fino que roda o
binário com `ulimit -v` e `timeout` padronizados (valores configuráveis por variável
de ambiente), usado tanto pelos testes E2E internos quanto por qualquer investigação
manual. Isso remove a necessidade de lembrar os limites manualmente (a causa direta
do incidente que travou a máquina de desenvolvimento durante a investigação deste
bug).

Critério de aceite: `scripts/run-gc-stress.sh` existe, tem teste de
caracterização em `tests/scripts/`, e é referenciado em
`specs/TESTING_STRATEGY.md` como o método recomendado para reproduzir bugs de GC
manualmente.

### P4 — Verificação estrutural de dominância para `Variable`s cacheadas (médio prazo)

Nenhum teste atual verifica a **propriedade estrutural** (dominância de definição)
diretamente — todos são testes de comportamento observável (saída do programa,
contagem de stats). Investigar se `cranelift_frontend`/`cranelift_codegen` expõe uma
forma de inspecionar o CLIF gerado (já usado internamente para debug) e adicionar um
teste que:

1. gera CLIF para um corpo com `if`/`cons` como o de P2;
2. verifica que a instrução de load inicial de `gc_sp_var` aparece no bloco de
   entrada da função (dominando todo o resto), não em um bloco de ramo.

Esse é o teste que verificaria a **causa raiz** diretamente, não só um sintoma
observável; é mais caro de escrever e mantido em segundo plano em relação a P1-P3.

Critério de aceite: falha determinística e imediata (sem precisar rodar o binário
sob GC_STRESS) se um futuro cache introduzir o mesmo padrão de inicialização
condicional.

### P5 — Nota na ADR-0018 (imediato, documental)

Adicionar a ADR-0018 uma seção "Lições" documentando o bug de #181, sua causa raiz, e
a referência a este documento — para que decisões futuras (D2-D5, ainda pendentes)
apliquem `ensure_sp_var()`-no-bloco-de-entrada como padrão desde o início, em vez de
reintroduzir a inicialização preguiçosa.

## Não objetivos

- Não propõe reescrever o cache de `gc_sp` para uma abordagem diferente (ex.: sempre
  ler/escrever o global) — isso reverteria o ganho de performance que motivou D1.
- Não propõe adicionar sanitizers Rust (Miri, etc.) para o codegen — o bug está na
  lógica de emissão de IR, não em unsafe Rust; ASan/UBSan (já cobertos por
  `test-runtime-sanitize`) não o alcançam porque o bug vive no lado Rust do codegen,
  não no runtime C em si.
- Não propõe mudar o `CLJN_GC_STRESS` nem o coletor.

## Referências

- Issue #181 / ADR-0018 (D1: cache de `gc_sp`).
- `src/compiler/clojure-codegen/src/lib.rs`: `enter_planned_frame`, `ensure_sp_var`,
  `flush_sp`, `sync_sp`.
- `src/compiler/clojure-native-cli/tests/e2e.rs`: `gc_correctness_under_stress`,
  `native_connector_bodies_and_gc_stress`.
- `src/compiler/clojure-codegen/tests/unit/lib/mod.rs`:
  `adr18_d1_gc_sp_flushed_once_per_call_not_per_push`.
