# Documentação

Este diretório reúne os guias correntes do `clojure-compiler` e a página publicada no
GitHub Pages. O repositório se chama `clojure-compiler`; o executável produzido se chama
`clojure-native`.

> Snapshot documentado: [`HEAD 476aefd`](https://github.com/EwertonDCSilv/clojure-compiler/commit/476aefd47bd01c4dca8b11f3e8009fbf2cd78d3c)
> em 2026-07-27. Consulte [SNAPSHOT.md](SNAPSHOT.md) para distinguir o HEAD auditado do
> commit do compilador medido.

## Comece aqui

| Documento | Conteúdo |
| --- | --- |
| [Visão geral](overview.md) | capacidades entregues, runtime, qualidade e limites atuais |
| [Uso do compilador](usage.md) | requisitos, Makefile, CLI, instalação, testes e benchmarks |
| [Arquitetura](architecture.md) | crates, pipeline AOT, modelo de valores, runtime e gates |
| [Snapshot](SNAPSHOT.md) | HEAD auditado, mudanças incluídas e benchmark de referência |
| [Especificações](../specs/README.md) | escopo da linguagem, decisões, planos e ADRs |
| [Conformidade](../specs/conformance/README.md) | contrato executável da matriz A–E |
| [Benchmarks](../benchmarks/README.md) | catálogo dos 90 casos, metodologia e resultados |

A apresentação pública fica em [index.html](index.html), com traduções em
[i18n.js](i18n.js). Ela inclui um painel interativo dos benchmarks Cormen e Cracking;
os SVGs versionados ficam em [`assets/benchmarks/`](assets/benchmarks/) e são
regenerados por `make benchmarks-charts`. Os gráficos permanecem neutros em relação ao
idioma; títulos, legendas e descrições acessíveis são traduzidos pelo HTML. O arquivo
`.nojekyll` permite que o conteúdo desta pasta seja publicado diretamente pelo GitHub
Pages.

## Fonte de verdade

- O comportamento entregue é definido pelo código e pelos casos `active` em
  [`tests/conformance/`](../tests/conformance).
- O [`Makefile`](../Makefile) é a entrada operacional recomendada para build, testes,
  cobertura, compatibilidade, benchmarks e instalação.
- As especificações também descrevem trabalho futuro. Itens marcados como `[FUTURO]`,
  `xfail` ou `pending` não representam funcionalidade disponível.
- Os READMEs em `benchmarks/*/results/` são fotografias de rodadas específicas. Seus
  comandos, ambiente e versões devem permanecer associados aos CSVs correspondentes.
- O marcador em [`SNAPSHOT.md`](SNAPSHOT.md) identifica o HEAD que foi auditado. O
  commit que grava uma revisão documental é posterior por definição e não substitui
  silenciosamente esse baseline.

## Validação rápida

Execute a partir da raiz do repositório:

```bash
make help
make quality
make compatibility
```

`make all` acrescenta cobertura e os 90 benchmarks. Consulte o
[guia de uso](usage.md) para requisitos opcionais e filtros.
