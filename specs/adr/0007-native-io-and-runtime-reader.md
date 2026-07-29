# ADR-0007 — I/O nativo atrás da ABI C e reader de runtime em Clojure

- Status: **aceita e implementada** (slurp/spit, streams de arquivo/string/bytes, paths, binding, reader de runtime; superfície `cljn.io`/`cljn.process` em progresso — #103)
- Data: 2026-07-27
- Relacionadas: [ADR-0002](0002-memory-management.md),
  [ADR-0003](0003-value-representation.md),
  [ADR-0004](0004-macro-execution.md),
  [ADR-0005](0005-bootstrap-strategy.md) e
  [IO_SPEC](../IO_SPEC.md)

## Contexto

O executável atual consegue escrever por `print`/`println`, mas ainda não possui um
modelo geral de streams, arquivos, paths, erros capturáveis ou reader em runtime.
Implementar cada função diretamente no codegen espalharia detalhes de descritores,
buffers e códigos do SO pelo frontend. Também duplicaria regras de lifecycle e
rooting, justamente onde falhas produzem leaks ou use-after-close.

O projeto já possui uma ABI C entre código gerado e runtime. O gate inicial precisa
ser Linux x86_64, síncrono e bloqueante, mas deve deixar uma fronteira que permita
outras plataformas sem alterar a semântica Clojure.

## Decisão

Handles, buffers, registro de recursos externos e syscalls ficam atrás da ABI C atual.
Primitivas da ABI são pequenas e orientadas a blocos: abrir/fechar, ler/escrever
buffers, seek, metadata e operações de filesystem. Elas nunca expõem `FILE *` ou file
descriptors como inteiros da linguagem.

APIs derivadas, normalização de opções, macros de lifecycle, EDN e o reader de código
em runtime são escritos em Clojure. O parser do compilador continua em Rust; o parser
de runtime não reutiliza sua implementação, mas ambos são validados pelo mesmo corpus
diferencial.

### Recursos externos e GC

Cada handle aberto é registrado numa tabela de recursos externos. O objeto gerenciado
mantém a identidade do registro, não o descritor cru. Isso impede coleta prematura
enquanto o recurso está aberto e torna possível auditar leaks ao terminar um teste.
`close!` fecha o recurso, marca o handle e remove o registro. Fechamento duplo é
idempotente.

Finalizers podem existir futuramente como proteção diagnóstica, mas não são requisito
de correção. O programa deve fechar explicitamente, preferencialmente por `with-open`.
Streams padrão são handles especiais não fecháveis.

### Buffering

Readers e writers mantêm buffers internos. Decoding UTF-8 incremental e pushback
ocorrem acima das leituras em bloco. Uma chamada ABI por byte ou caractere é proibida
pelo critério de performance e deve ser detectada por instrumentação nos testes.
Escritas parciais e interrupções são repetidas até conclusão ou erro categorizado.

### Erros e unwind

A ABI converte resultados do SO em um resultado interno estável. A camada Clojure
lança `cljn.io/IOException`, preservando categoria, operação, path, código do SO e
mensagem em `ex-data`. Isso depende de exceções nativas capturáveis e
`try/catch/finally`, que são bloqueantes para o gate.

### Compatibilidade deliberada

- `cljn.io` substitui objetos e protocolos de `clojure.java.io`;
- texto é somente UTF-8 no primeiro gate;
- Linux x86_64 é a primeira plataforma bloqueante;
- URL, socket, subprocesso e terminal interativo não fazem parte desta decisão;
- `Path` preserva bytes nativos e pode não ter representação textual;
- macros continuam sem acesso a I/O por padrão, embora programas finais usem as
  permissões normais do processo.

## Alternativas consideradas

| Alternativa | Resultado | Motivo |
| --- | --- | --- |
| Chamadas libc emitidas diretamente pelo codegen | rejeitada | acopla frontend ao SO, replica tratamento de erro/rooting e dificulta outras plataformas |
| Biblioteca Rust estática adicional dentro do executável | rejeitada nesta fase | cria uma segunda fronteira de runtime e ownership sem vantagem para as syscalls simples do primeiro gate |
| Segundo reader escrito em C | rejeitada | aumenta código unsafe e duplica parsing de alto nível onde Clojure é mais testável |
| Um único parser Rust compartilhado por build e runtime | adiada | exigiria expor objetos/controle Rust ao executável e acoplar o runtime ao compilador |
| ABI C para primitivas + camadas derivadas em Clojure | escolhida | preserva a fronteira atual, concentra unsafe e permite bootstrap progressivo |

## Consequências

Positivas:

- syscalls, buffers e descritores ficam numa superfície pequena e auditável;
- a maior parte da semântica pode ser testada como Clojure compilado;
- o registro de handles permite gate objetivo de leak;
- parsers independentes reduzem risco de um erro compartilhado passar despercebido.

Custos:

- existem duas implementações de parser a manter;
- exceções e Vars dinâmicas precisam chegar antes do I/O completo;
- o runtime C ganha estado externo e caminhos de erro mais complexos;
- compatibilidade com bibliotecas que esperam classes Java requer adapters ou permanece
  fora do nível alcançado.

## Verificação

A decisão é aceita quando:

1. a matriz descrita em [IO_SPEC](../IO_SPEC.md) estiver `active`;
2. testes contarem chamadas ABI e comprovarem buffering;
3. o registro de handles estiver vazio após caminhos normais e excepcionais;
4. GC stress e sanitizers cobrirem handles, buffers e unwind;
5. o reader de runtime e o reader Rust passarem o mesmo corpus;
6. operações recursivas, symlinks, UTF-8 inválido e escritas parciais tiverem casos
   explícitos.

Uma futura migração de primitivas para Rust, adoção de I/O assíncrono ou unificação dos
parsers exige nova ADR e medição que justifique substituir esta decisão.
