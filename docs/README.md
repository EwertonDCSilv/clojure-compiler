# Documentação

Este diretório reúne os guias correntes do `clojure-compiler` e a página publicada no
GitHub Pages. O repositório se chama `clojure-compiler`; o executável produzido se chama
`clojure-native`.

## Comece aqui

| Documento | Conteúdo |
| --- | --- |
| [Visão geral](overview.md) | capacidades entregues, runtime, qualidade e limites atuais |
| [Uso do compilador](usage.md) | requisitos, Makefile, CLI, instalação, testes e benchmarks |
| [Arquitetura](architecture.md) | crates, pipeline AOT, modelo de valores, runtime e gates |
| [Especificações](../specs/README.md) | escopo da linguagem, decisões, planos e ADRs |
| [Conformidade](../specs/conformance/README.md) | contrato executável da matriz A–E |
| [Benchmarks](../benchmarks/README.md) | catálogo dos 90 casos, metodologia e resultados |

A apresentação pública fica em [index.html](index.html), com traduções em
[i18n.js](i18n.js). O arquivo `.nojekyll` permite que o conteúdo desta pasta seja
publicado diretamente pelo GitHub Pages.

## Fonte de verdade

- O comportamento entregue é definido pelo código e pelos casos `active` em
  [`tests/conformance/`](../tests/conformance).
- O [`Makefile`](../Makefile) é a entrada operacional recomendada para build, testes,
  cobertura, compatibilidade, benchmarks e instalação.
- As especificações também descrevem trabalho futuro. Itens marcados como `[FUTURO]`,
  `xfail` ou `pending` não representam funcionalidade disponível.
- Os READMEs em `benchmarks/*/results/` são fotografias de rodadas específicas. Seus
  comandos, ambiente e versões devem permanecer associados aos CSVs correspondentes.

## Validação rápida

Execute a partir da raiz do repositório:

```bash
make help
make quality
make compatibility
```

`make all` acrescenta cobertura e os 90 benchmarks. Consulte o
[guia de uso](usage.md) para requisitos opcionais e filtros.
