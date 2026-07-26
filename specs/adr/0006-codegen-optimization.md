# ADR-0006 — Otimização do codegen numérico e do rooting de GC

- **Status:** Proposto
- **Contexto:** o backend Cranelift atual representa valores compilados em uma palavra
  tagged, mas envia toda aritmética e comparação à ABI C (`cljn_add`, `cljn_le`,
  `cljn_inc` etc.). Além disso, aplica rooting eager: todo resultado de expressão chama
  `cljn_gc_push`, todo descarte chama `cljn_gc_popn` e todo binding local chama
  `cljn_gc_set`. Em loops numéricos sem alocação, essas chamadas dominam o custo apesar de
  não haver oportunidade de coleta. O plano detalhado está em
  [optime.md](../optime.md).

Esta ADR refina o codegen escolhido na ADR-0001 e a manutenção do shadow-stack da
ADR-0002. Ela **não** troca o backend, a representação externa de valores nem o coletor.

## Forças de decisão

- Preservar a semântica dinâmica: operandos podem não ser inteiros e operações podem
  falhar por tipo ou overflow.
- Manter o GC tracing preciso sem escanear a pilha nativa.
- Retirar calls e tráfego de memória de loops que não alocam.
- Continuar portátil entre os targets suportados pelo Cranelift, sem assembly específico.
- Permitir promoção futura para bigint e evolução do runtime por meio de slow paths.
- Fazer a correção depender de contratos explícitos e testáveis, não de otimizações
  incidentais do compilador C ou do Cranelift.

## Alternativas consideradas

| Alternativa | Vantagens | Desvantagens | Veredito |
| --- | --- | --- | --- |
| Manter toda operação no runtime e rooting por expressão | codegen simples; sem novos guards | milhões de calls; impede otimização do loop; roots atualizados onde GC não pode rodar | rejeitada |
| Confiar em inlining/LTO do runtime C | pouca mudança no frontend | dependente de toolchain/link; não resolve o protocolo eager de roots; sem garantia cross-platform | rejeitada |
| Gerar apenas aritmética direta e manter rooting eager | ganho parcial e implementação menor | loops continuam fazendo `gc_set/push/popn`; não resolve o segundo gargalo | etapa intermediária, não solução final |
| Inferir estaticamente todos os tipos e gerar somente código especializado | hot path mínimo | incompatível com o dinamismo sem versionamento/deopt; complexidade desproporcional | rejeitada como requisito |
| Substituir o shadow-stack por stack maps do backend | roots precisos sem stores manuais | dependência maior do backend e da plataforma; integração ainda não validada | futuro |
| Fast paths guardados + slow paths e rooting somente em safepoints | preserva dinamismo e GC; remove custo do caminho comum | aumenta o controle de fluxo e exige metadados de efeito/liveness corretos | **escolhida** |

## Decisão

Adotar em conjunto:

1. **Fast paths Cranelift guardados para fixnums.**
2. **Classificação explícita de valores e efeitos de calls.**
3. **Shadow-stack com slots fixos, atualizado somente em safepoints conforme liveness.**
4. **Slow paths de runtime para tipo inválido, overflow e extensões numéricas futuras.**

As quatro partes formam uma única decisão: aritmética direta sem revisão do rooting deixa
grande parte do custo no loop; eliminação de roots sem efeitos explícitos compromete a
correção do coletor.

### Fast path numérico

Para `+`, `-`, `*`, `<`, `<=`, `>`, `>=`, `inc` e `dec`, o codegen deve:

1. testar a tag quando o operando não estiver comprovadamente fixnum;
2. remover a tag;
3. emitir a operação inteira/comparação diretamente em Cranelift;
4. validar o intervalo de fixnum nos operadores que produzem número;
5. recolocar a tag ou selecionar `TRUEV`/`FALSEV`;
6. desviar ao slow path quando uma guarda falhar.

`=` pode usar identidade tagged como fast path, preservando a igualdade estrutural no
runtime. `quot` e `mod` entram depois de cobertos divisão por zero, mínimo dividido por
`-1` e a regra de sinal de `mod`.

O intervalo de fixnum em alvo de 64 bits é:

```text
FIXNUM_MIN = -(1 << 62)
FIXNUM_MAX =  (1 << 62) - 1
```

Checar somente overflow de `i64` é insuficiente. O resultado deve caber no intervalo
acima antes do retagging. A implementação C do slow path deve codificar valores por
operações unsigned para não executar left shift de signed negativo.

### Efeitos e fatos de valor

Toda operação importada ou gerada recebe metadados:

```text
CallEffect = NoSafepoint | MaySafepoint
ValueKind  = Fixnum | Immediate | MaybeHeap
```

- `NoSafepoint` não pode alcançar alocação ou coleta.
- `MaySafepoint` inclui construtores, operações persistentes que alocam e calls Clojure
  diretas ou indiretas.
- Literais imediatos e resultados numéricos validados não precisam de root.
- Valor de tipo desconhecido é `MaybeHeap`; a análise deve permanecer conservadora.

Adicionar uma função ao runtime exige declarar seu efeito. Se uma função
`NoSafepoint` passar a alocar, seu efeito deve mudar no mesmo commit.

Fatos simples propagam por literais, `let`, ramos e `loop/recur`. Guardas só podem ser
eliminadas quando o fato `Fixnum` for provado em todos os caminhos. Unboxing permanente
de locais de loop não faz parte desta decisão inicial; depende de benchmark próprio.

### Rooting em safepoints

Substituir a invariante:

> todo resultado de expressão está no topo do shadow-stack

por:

> antes de cada safepoint, todo valor `MaybeHeap` vivo durante ou depois dele está em um
> root slot visível ao coletor.

Consequências operacionais:

- produzir uma expressão ou vincular um local não escreve automaticamente no
  shadow-stack;
- bindings atualizam a variável Cranelift e deixam o root logicamente dirty;
- antes de `MaySafepoint`, o codegen grava apenas roots vivos e dirty;
- entre safepoints não há sincronização de roots;
- roots mortos podem ser limpos de forma lazy antes do próximo safepoint;
- estados de `if`, retorno e backedge de `recur` são reconciliados conservadoramente.

Cada função terá um plano de roots com slots fixos e reutilizáveis. `gc_enter` reserva o
frame e devolve sua base endereçável; o código gerado usa stores Cranelift diretos.
`gc_leave` encerra o frame. Os helpers `cljn_gc_push`, `cljn_gc_popn` e `cljn_gc_set`
deixam de ser emitidos pelo novo codegen e são removidos da ABI somente após a migração.

O modelo é seguro porque, conforme ADR-0002:

- a coleta é single-thread e só ocorre em safepoints conhecidos;
- o GC é não-móvel, então valores em registradores continuam válidos após a coleta;
- não existe coleta assíncrona entre caller e callee.

Calls Clojure são `MaySafepoint`: valores do caller vivos após a call são flushados antes
dela. O callee registra seus parâmetros e os flushará antes de seu primeiro safepoint.
Argumentos anteriores também são rooteados quando a avaliação dos argumentos seguintes
puder alocar.

Uma futura coleta móvel ou assíncrona exige reabrir esta ADR: seria necessário recarregar
valores atualizados, usar stack maps ou adotar outro protocolo equivalente.

## Consequências

### Positivas

- Loops inteiros sem alocação podem executar sem calls de runtime ou stores de GC por
  iteração.
- A semântica dinâmica permanece no slow path; o fast path não exige um sistema de tipos
  estático.
- Menos calls cruzam a ABI C e o Cranelift passa a enxergar as operações do hot path.
- O protocolo de safepoints fica explícito, revisável e reutilizável por novas
  primitivas.
- A precisão do GC é preservada sem escanear a pilha nativa.

### Negativas

- `clojure-codegen` passa a manter fatos de valor, efeitos, liveness e estado de root por
  bloco.
- Guards e slow blocks aumentam inicialmente o tamanho do código.
- Uma classificação incorreta de `NoSafepoint` pode causar use-after-free.
- A ABI interna do shadow-stack muda para expor slots endereçáveis ao código gerado.
- O desenho depende das propriedades não-móvel e single-thread do GC atual.

### Mitigações

- Metadados de efeito centralizados e obrigatórios.
- Estado conservador quando liveness ou tipo não forem provados.
- Slow blocks compartilhados e hoisting de guards invariantes.
- Testes e2e sob `CLJN_GC_STRESS=1`, ASan e UBSan.
- Casos sentinela com locals, temporários, argumentos e capturas heap vivos através de
  alocações.
- Testes estruturais sobre CLIF e objeto gerado, independentes de tempo de benchmark.
- Se a liveness deixar de ser um prepass local verificável, movê-la para a LIR/ANF
  prevista em `COMPILER_PIPELINE.md`, em vez de criar uma SSA paralela no backend.

## Critérios de validação

A decisão é considerada implementada quando:

- o bloco quente de `examples/loop-benchmark.clj` não chama `cljn_add`, `cljn_le`,
  `cljn_inc` ou equivalentes;
- loops sem alocação não emitem calls ou stores de rooting por iteração;
- o objeto gerado não importa `cljn_gc_push`, `cljn_gc_popn` ou `cljn_gc_set`;
- erros de tipo, overflow e divisão por zero preservam o comportamento definido;
- toda a suíte e2e passa normalmente e com `CLJN_GC_STRESS=1`;
- benchmarks alocadores não apresentam regressão material e o ganho numérico é registrado
  contra um baseline reproduzível.

## Relação com outras decisões

- **ADR-0001:** mantém Cranelift como backend e passa a usá-lo para o hot path numérico.
- **ADR-0002:** mantém GC preciso e shadow-stack; refina sua atualização de eager para
  safepoint/liveness.
- **ADR-0003:** não altera a representação pública; especializa a variante tagged do
  runtime compilado.
- **Plano de execução:** [optime.md](../optime.md).
