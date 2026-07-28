# Spec — Connector HTTP nativo desacoplado para aplicações Pedestal-compatible

- **Status:** proposta
- **Data:** 2026-07-28
- **Produto:** `clojure-native`
- **Repositório:** `clojure-compiler`
- **Nome provisório do componente:** `cljn.pedestal.connector`
- **Alvo inicial:** Linux x86_64, HTTP/1.1, síncrono, bloqueante
- **Compatibilidade inicial:** subconjunto explícito do modelo de connectors e interceptors do Pedestal 0.8
- **Documentos relacionados:**
  - [`README.md`](README.md)
  - [`COMPATIBILITY_SPEC.md`](COMPATIBILITY_SPEC.md)
  - [`IO_SPEC.md`](IO_SPEC.md)
  - [`RUNTIME_SPEC.md`](RUNTIME_SPEC.md)
  - [`NATIVE_INTEROP.md`](NATIVE_INTEROP.md)
  - [`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md)
  - [`adr/0007-native-io-and-runtime-reader.md`](adr/0007-native-io-and-runtime-reader.md)
  - [`adr/0013-compiled-clojure-pedestal-native-connector.md`](adr/0013-compiled-clojure-pedestal-native-connector.md)

> Esta spec define um alvo futuro e incremental. A presença de uma API, namespace ou
> comportamento neste documento não significa que ele já esteja implementado. O estado
> executável continua sendo determinado pela matriz em `tests/conformance/`.

---

## 1. Resumo executivo

O projeto deve oferecer um connector HTTP nativo capaz de transformar requisições de
rede em mapas Clojure, executar uma cadeia síncrona de interceptors e transformar o mapa
de resposta resultante em uma resposta HTTP, sem depender de JVM, Servlet, Jetty,
HTTP-Kit ou interop Java.

O connector deve ser **desacoplado** em quatro sentidos:

1. não deve incorporar regras de roteamento no servidor HTTP;
2. não deve conhecer interceptors específicos da aplicação;
3. não deve expor sockets, file descriptors ou estruturas do backend à linguagem;
4. não deve depender de uma implementação específica de parser ou event loop.

O primeiro corte vertical prioriza correção, contrato e mensuração. Ele será
single-threaded, aceitará apenas HTTP/1.1 sem streaming e manterá request e response body
inteiros em memória, com limites configuráveis.

A compatibilidade com Pedestal será inicialmente **source-compatible por subconjunto**,
não uma afirmação de que o código upstream completo compila no `clojure-native`.

---

## 2. Contexto

O Pedestal 0.8 separa o lifecycle da aplicação do adaptador de rede através do conceito
de connector. O contrato observável principal contém operações equivalentes a:

- iniciar o connector;
- interromper o connector;
- executar uma requisição de teste sem depender da rede.

O modelo central de processamento do Pedestal também é adequado a um runtime nativo:

- request e response são mapas;
- interceptors são dados contendo callbacks;
- o contexto atravessa estágios `:enter`, `:leave` e `:error`;
- rotas podem ser expressas como dados;
- o adaptador de rede fica na borda do sistema.

Entretanto, o código upstream atual depende de recursos ainda ausentes ou deliberadamente
fora do escopo do `clojure-native`, incluindo Java interop, classes JVM, `core.async`,
Vars dinâmicas completas, namespaces multi-arquivo, dependências Maven e bibliotecas do
ecossistema JVM.

A estratégia desta spec é preservar a semântica útil e a forma da API, sem tentar portar
toda a implementação upstream como primeira etapa.

---

## 3. Problema

Hoje não existe uma forma de executar uma aplicação HTTP reconhecível como Pedestal em
um executável gerado pelo `clojure-native`.

As lacunas principais são:

- ausência de uma API de sockets e servidor HTTP;
- ausência de um contrato nativo de connector;
- ausência de um executor de interceptors síncronos compatível;
- ausência de request/response maps normalizados;
- ausência de lifecycle observável e testável;
- ausência de um projeto multi-arquivo e de resolução de source dependencies;
- ausência de uma suíte diferencial contra Pedestal/JVM;
- ausência de benchmarks HTTP reproduzíveis.

Tentar compilar Jetty ou HTTP-Kit não é uma solução aceitável, pois ambos pertencem ao
mundo JVM. Tentar incorporar rede diretamente ao codegen também viola a fronteira entre
frontend, runtime e plataforma.

---

## 4. Objetivos

### 4.1 Objetivos funcionais

O connector deve:

1. receber uma configuração declarativa;
2. criar um servidor HTTP nativo;
3. converter uma requisição HTTP em um request map;
4. construir o contexto inicial;
5. executar uma cadeia síncrona de interceptors;
6. obter um response map válido;
7. serializar a resposta para HTTP;
8. fornecer lifecycle explícito;
9. executar `test-request` sem abrir porta de rede;
10. produzir erros categorizados e observáveis.

### 4.2 Objetivos de arquitetura

- Manter o backend de rede atrás de uma ABI estável.
- Permitir substituir parser, socket loop ou backend sem alterar código de aplicação.
- Manter roteamento e interceptors fora do núcleo de rede.
- Compartilhar a mesma normalização entre tráfego real e `test-request`.
- Tornar limites de segurança explícitos e configuráveis.
- Permitir evolução posterior para multiprocessamento, concorrência e streaming.

### 4.3 Objetivos de compatibilidade

No subconjunto declarado como `active`, uma aplicação deve poder usar conceitos e formas
familiares a Pedestal:

- connector map;
- interceptor maps ou records;
- `:enter`, `:leave` e `:error`;
- request map compatível com o subconjunto Ring;
- response map com `:status`, `:headers` e `:body`;
- lifecycle `start!`, `stop!` e `test-request`;
- rotas declarativas em um módulo separado.

---

## 5. Não objetivos do primeiro gate

Os seguintes itens ficam explicitamente fora do primeiro gate:

- Servlet API;
- Jetty;
- HTTP-Kit;
- Java interop;
- HTTP/2 ou HTTP/3;
- TLS embutido;
- WebSockets;
- Server-Sent Events;
- streaming de request ou response;
- chunked response gerada pela aplicação;
- async interceptors;
- `core.async`;
- thread pool dentro do processo;
- hot reload;
- REPL em runtime;
- descoberta dinâmica de namespaces;
- carregamento dinâmico de rotas;
- compatibilidade binária com bibliotecas JVM;
- middleware Ring arbitrário;
- compressão automática;
- multipart;
- sessão, cookies assinados, Transit ou JSON no núcleo do connector;
- OpenTelemetry no primeiro corte vertical.

Esses itens podem ser adicionados por specs e ADRs posteriores. Nenhum deles deve ser
simulado silenciosamente.

---

## 6. Princípios normativos

### 6.1 Sem compatibilidade implícita

O projeto não deve declarar “suporte a Pedestal” sem informar:

- versão de referência;
- módulos ou namespaces cobertos;
- recursos suportados;
- recursos rejeitados;
- divergências deliberadas;
- casos de conformidade executáveis.

### 6.2 Falhar de forma explícita

Uma opção ou construção não suportada deve produzir erro de build ou runtime
categorizado. O connector não deve ignorar silenciosamente:

- configuração desconhecida;
- response body não suportado;
- interceptor assíncrono;
- resposta inválida;
- headers inválidos;
- método inválido;
- framing HTTP ambíguo.

### 6.3 Mesma semântica para teste e rede

`test-request` e uma requisição recebida por socket devem atravessar:

1. a mesma validação do request map;
2. a mesma montagem de contexto;
3. a mesma cadeia de interceptors;
4. a mesma validação do response map;
5. a mesma normalização final da resposta.

Somente parsing e serialização HTTP ficam fora de `test-request`.

### 6.4 Rede não conhece aplicação

O backend HTTP recebe uma função de dispatch com contrato estável. Ele não conhece:

- rotas;
- nomes de namespaces;
- interceptors específicos;
- regras de autenticação;
- formato JSON ou EDN;
- domínio da aplicação.

### 6.5 Primeiro gate coerente com o runtime atual

Como o GC atual é single-threaded, o primeiro servidor deve executar no modo
`:single-thread`, com no máximo uma cadeia de aplicação em execução por processo.
Concorrência por threads fica bloqueada até existir um modelo de runtime e GC seguro.

---

## 7. Terminologia

| Termo | Definição |
| --- | --- |
| Connector | Adaptador que liga lifecycle, backend HTTP e dispatch da aplicação |
| Backend HTTP | Implementação de socket, parsing, framing e escrita da resposta |
| Dispatch | Função que recebe request map e devolve response map |
| Context | Mapa que atravessa a cadeia de interceptors |
| Interceptor | Valor com callbacks opcionais `:enter`, `:leave` e `:error` |
| Request map | Representação Clojure normalizada de uma requisição |
| Response map | Representação Clojure normalizada de uma resposta |
| Provider | Implementação substituível do backend HTTP atrás da ABI |
| Gate | Conjunto mínimo de critérios bloqueantes para declarar uma fase completa |

---

## 8. Arquitetura de alto nível

```text
┌─────────────────────────────────────────────────────────────┐
│ Aplicação Clojure compilada                                │
│ handlers, rotas, interceptors                              │
└──────────────────────────┬──────────────────────────────────┘
                           │ maps + functions
┌──────────────────────────▼──────────────────────────────────┐
│ Camada Pedestal-compatible                                │
│ connector map, chain síncrona, request/response validation │
└──────────────────────────┬──────────────────────────────────┘
                           │ dispatch(request) -> response
┌──────────────────────────▼──────────────────────────────────┐
│ Connector nativo                                           │
│ lifecycle, limites, eventos, test-request                   │
└──────────────────────────┬──────────────────────────────────┘
                           │ ABI opaca
┌──────────────────────────▼──────────────────────────────────┐
│ Provider HTTP                                               │
│ socket, accept, parse, timeout, serialize, close            │
└──────────────────────────┬──────────────────────────────────┘
                           │ syscalls
┌──────────────────────────▼──────────────────────────────────┐
│ Sistema operacional                                        │
└─────────────────────────────────────────────────────────────┘
```

### 8.1 Componentes

#### A. `cljn.pedestal.chain`

Implementa somente a cadeia síncrona de interceptors.

#### B. `cljn.pedestal.connector`

Cria configuração, valida opções, inicializa o backend, monta o dispatch e controla
lifecycle.

#### C. `cljn.http.request`

Normaliza e valida request maps.

#### D. `cljn.http.response`

Valida e normaliza response maps.

#### E. `cljn.http.runtime`

Expõe primitivas nativas internas para o connector. Não é API pública de aplicação.

#### F. provider HTTP

Implementa o transporte no runtime C atrás da ABI existente, conforme a ADR-0013.

#### G. `cljn.pedestal.route`

Módulo separado e opcional para o primeiro roteador estático compatível. O connector não
depende dele.

---

## 9. Estratégia de compatibilidade

A implementação será dividida em quatro camadas de compatibilidade.

### 9.1 Camada P0 — API nativa própria

Namespaces `cljn.*`, compiláveis sem dependências externas e sem afirmar que módulos
upstream foram compilados.

### 9.2 Camada P1 — Source compatibility

Formas equivalentes ao uso básico do Pedestal, com pequenas mudanças de namespace.

Exemplo:

```clojure
(ns hello
  (:require [cljn.pedestal.connector :as conn]
            [cljn.pedestal.route :as route]))
```

### 9.3 Camada P2 — Compatibilidade com namespaces upstream

Após source dependencies, namespaces multi-arquivo, macros e biblioteca padrão
suficientes, o projeto poderá compilar um subconjunto selecionado do Pedestal upstream.

P2 não é requisito para o primeiro servidor funcional.

### 9.4 Camada P3 — Implementação direta de `PedestalConnector`

Quando o protocolo upstream puder ser compilado, o connector nativo poderá implementar
diretamente o contrato de `io.pedestal.service.protocols/PedestalConnector`.

P3 exige uma ADR específica e uma matriz diferencial contra a versão upstream escolhida.

---

## 10. Contrato público do connector

### 10.1 Namespace provisório

```clojure
cljn.pedestal.connector
```

### 10.2 Construção da configuração

```clojure
(default-connector-map port)
(default-connector-map host port)
```

Resultado mínimo:

```clojure
{:host "127.0.0.1"
 :port 8080
 :join? false
 :mode :single-thread
 :interceptors []
 :initial-context {}
 :limits {:request-line-bytes 8192
          :header-bytes 32768
          :header-count 100
          :body-bytes 1048576}
 :timeouts {:read-ms 5000
            :write-ms 5000
            :idle-ms 15000}}
```

### 10.3 Operações públicas

```clojure
(create-connector connector-map)
(start! connector)
(serve-one! connector)
(serve! connector)
(stop! connector)
(test-request connector request-map)
(connector-state connector)
(connector-address connector)
(connector-stats connector)
```

### 10.4 Operações de composição

```clojure
(with-interceptor connector-map interceptor)
(with-interceptors connector-map interceptors)
(with-initial-context connector-map context)
(with-dispatch connector-map dispatch-fn)
(with-limits connector-map limits)
(with-timeouts connector-map timeouts)
(with-event-handler connector-map event-fn)
```

`with-dispatch` e `with-interceptors` são mutuamente exclusivos no primeiro gate:

- `with-dispatch` recebe uma função request -> response;
- `with-interceptors` cria internamente um dispatch baseado na cadeia.

Configurar ambos deve gerar erro `:ambiguous-dispatch`.

No modo nativo single-thread, `start!` apenas abre o listener quando `:join?` é falso;
o chamador precisa executar `serve!` ou `serve-one!`. Não existe processamento em
background implícito. Com `:join? true`, `start!` entra em `serve!` depois do bind,
conforme decidido na [ADR-0013](adr/0013-compiled-clojure-pedestal-native-connector.md).

### 10.5 Representação do connector

O connector deve ser um valor opaco para a aplicação. Sua impressão deve evitar ponteiros,
file descriptors ou segredos:

```clojure
#cljn/http-connector
{:state :running
 :host "127.0.0.1"
 :port 8080
 :mode :single-thread}
```

Igualdade de connectors é por identidade.

---

## 11. Connector map

### 11.1 Chaves obrigatórias

| Chave | Tipo | Regra |
| --- | --- | --- |
| `:host` | string | IPv4 literal ou `localhost`; DNS fica fora de P1 |
| `:port` | fixnum | 0 a 65535; 0 permite porta efêmera |
| `:mode` | keyword | somente `:single-thread` no primeiro gate |
| `:join?` | boolean | bloqueia `start!` até `stop!` quando verdadeiro |
| `:initial-context` | map | copiado para cada requisição |
| `:limits` | map | limites de framing e memória |
| `:timeouts` | map | timeouts em milissegundos |

### 11.2 Chaves opcionais

| Chave | Tipo | Uso |
| --- | --- | --- |
| `:interceptors` | vector | cadeia estática de interceptors |
| `:dispatch` | function | dispatch direto |
| `:event-handler` | function | recebe eventos internos normalizados |
| `:server-header` | string ou nil | header `Server`; nil omite |
| `:reuse-address?` | boolean | habilita opção equivalente do SO |
| `:backlog` | fixnum | fila de conexões pendentes |

### 11.3 Configuração desconhecida

Chaves desconhecidas no namespace público do connector devem falhar por padrão:

```clojure
{:kind :unknown-connector-option
 :option :threads}
```

Uma futura opção `:allow-unknown-options?` não faz parte do primeiro gate.

### 11.4 Porta efêmera

Quando `:port` for zero, o provider escolhe a porta. Após `start!`,
`connector-address` deve retornar a porta real:

```clojure
{:host "127.0.0.1"
 :port 43127}
```

---

## 12. Lifecycle

### 12.1 Estados

```text
:created -> :starting -> :running -> :stopping -> :stopped
                    \-> :failed
```

### 12.2 Regras

- `create-connector` retorna estado `:created`.
- `start!` em `:created` ou `:stopped` inicia o servidor.
- `start!` em `:running` falha com `:already-started`.
- `stop!` em `:running` solicita parada e fecha o listener.
- `stop!` em `:created` ou `:stopped` é idempotente.
- `test-request` funciona em qualquer estado não destruído.
- falha durante bind produz estado `:failed` e preserva erro categorizado.
- um connector em `:failed` pode ser reiniciado somente após `stop!` ou `reset!` futuro.

### 12.3 `join?`

Quando `:join? true`, `start!` bloqueia a thread corrente até:

- `stop!` ser chamado por mecanismo externo permitido;
- o servidor falhar;
- o processo receber sinal tratado pelo launcher.

Como o primeiro gate não possui concorrência Clojure, o uso primário de `join?` é em
`-main`, com sinais do processo encaminhados pelo runtime.

### 12.4 Sinais

No Linux, o launcher deve poder converter `SIGINT` e `SIGTERM` em solicitação de parada
graciosa. Handlers de sinal não executam código Clojure diretamente; eles apenas alteram
estado seguro consultado pelo loop do servidor.

---

## 13. Request map

### 13.1 Subconjunto obrigatório

```clojure
{:server-port 8080
 :server-name "localhost"
 :remote-addr "127.0.0.1"
 :uri "/greet"
 :query-string "name=Ada"
 :scheme :http
 :request-method :get
 :protocol "HTTP/1.1"
 :headers {"host" "localhost:8080"
           "accept" "*/*"}
 :body nil}
```

### 13.2 Regras de normalização

- nomes de headers são convertidos para ASCII lowercase;
- valores de headers permanecem strings;
- `:uri` contém o path bruto validado, sem query string;
- `:query-string` é string ou nil;
- método é convertido para keyword lowercase quando reconhecido;
- método sintaticamente válido, porém desconhecido, pode virar keyword própria;
- `:scheme` é `:http` no primeiro gate;
- `:protocol` é exatamente `"HTTP/1.0"` ou `"HTTP/1.1"`;
- `:body` é nil, `Bytes` ou string conforme política configurada.

### 13.3 Corpo da requisição

No primeiro gate, o corpo é lido integralmente antes do dispatch.

Configuração provisória:

```clojure
{:request-body :bytes}
```

Valores possíveis:

- `:bytes`: entrega `Bytes`;
- `:string`: exige UTF-8 válido e entrega string;
- `:discard`: consome e entrega nil.

O padrão é `:bytes`. Corpo acima de `:body-bytes` recebe resposta 413 sem chamar a
aplicação.

### 13.4 Headers repetidos

No primeiro gate:

- `content-length` repetido só é aceito quando todos os valores numéricos são idênticos;
- `cookie` pode ser unido por `"; "`;
- headers listáveis podem ser unidos por `", "`;
- `set-cookie` não é válido em request;
- headers não combináveis devem ser preservados por representação futura, portanto são
  rejeitados no primeiro gate se repetidos.

### 13.5 Path e percent encoding

O connector preserva o path bruto validado em `:uri`. Decodificação de path params
pertence ao roteador ou a um interceptor separado.

Requisições com percent encoding malformado devem ser rejeitadas com 400 antes da
aplicação.

---

## 14. Response map

### 14.1 Forma mínima

```clojure
{:status 200
 :headers {"content-type" "text/plain; charset=utf-8"}
 :body "Hello, world!\n"}
```

### 14.2 Validação

- `:status` é fixnum entre 100 e 599;
- `:headers` é map de string para string;
- nomes de headers devem ser tokens HTTP válidos;
- valores não podem conter CR ou LF;
- `:body` é nil, string ou `Bytes`;
- chaves desconhecidas são preservadas no contexto, mas ignoradas pelo serializer;
- ausência de `:status` torna a resposta inválida;
- ausência de `:headers` equivale a `{}`;
- ausência de `:body` equivale a nil.

### 14.3 Content-Length

O serializer calcula `Content-Length` para string, `Bytes` ou body nil.

Se a aplicação fornecer `content-length`:

- valor igual ao tamanho calculado é aceito;
- valor diferente gera erro `:content-length-mismatch` e resposta 500;
- valor inválido gera erro `:invalid-response-header`.

### 14.4 Transfer-Encoding

A aplicação não pode definir `transfer-encoding` no primeiro gate. A presença desse
header gera erro de resposta.

### 14.5 Respostas sem body

Respostas a `HEAD` e status 1xx, 204 e 304 não enviam body. O comprimento pode representar
o body que seria enviado em `GET` somente quando a semântica HTTP permitir.

### 14.6 Resposta inválida

Uma resposta inválida deve:

1. emitir evento `:invalid-response`;
2. descartar headers e body inválidos;
3. retornar resposta 500 mínima;
4. não vazar detalhes internos ao cliente.

Resposta padrão:

```clojure
{:status 500
 :headers {"content-type" "text/plain; charset=utf-8"
           "connection" "close"}
 :body "Internal Server Error\n"}
```

---

## 15. Modelo de interceptors

### 15.1 Forma

Um interceptor pode ser map ou record com:

```clojure
{:name :app/example
 :enter enter-fn
 :leave leave-fn
 :error error-fn}
```

Callbacks ausentes ou nil são ignorados.

### 15.2 Contexto inicial

```clojure
{:request request-map
 :response nil
 :bindings nil
 :cljn.pedestal.chain/queue ...
 :cljn.pedestal.chain/stack ...
 :cljn.pedestal.chain/execution-id 42}
```

As chaves internas da chain não fazem parte do contrato de aplicação, salvo através de
funções públicas de inspeção.

### 15.3 Semântica síncrona

1. interceptors são executados em ordem no estágio `:enter`;
2. cada callback recebe o contexto e deve devolver um map;
3. ao finalizar `:enter`, a pilha é executada em ordem inversa no estágio `:leave`;
4. ao ocorrer erro, a execução muda para `:error` em ordem inversa;
5. um `:error` pode resolver o erro ao devolver contexto sem a chave interna de erro;
6. erro não resolvido é propagado ao connector;
7. o connector converte erro não tratado em resposta 500.

### 15.4 Terminação antecipada

A chain fornece:

```clojure
(terminate context)
(terminate-when context predicate)
```

O connector deve incluir por padrão um terminador que inicia `:leave` quando existe uma
response válida no contexto.

### 15.5 Async proibido no primeiro gate

Se callback devolver qualquer valor diferente de map, o runtime gera:

```clojure
{:kind :invalid-interceptor-result
 :stage :enter
 :interceptor :app/example
 :value-type :unsupported}
```

Canais, promises, futures ou callbacks de continuação não são aceitos. O projeto não deve
interpretar nil como execução assíncrona.

### 15.6 Dynamic bindings

O caminho compilado já oferece `binding` para Vars conhecidas pelo compilador, mas não
possui o mapa geral de Vars exigido pela semântica de `:bindings` do Pedestal. A chave
permanece reservada no P1. Seu uso deve falhar explicitamente com
`:dynamic-bindings-unsupported`; isso não bloqueia connector, chain ou roteamento.

---

## 16. Dispatch

### 16.1 Contrato direto

```clojure
(fn [request-map] response-map)
```

### 16.2 Contrato por interceptors

O connector transforma a cadeia em dispatch equivalente a:

```clojure
(fn [request]
  (let [context (chain/execute
                  (assoc initial-context :request request)
                  interceptors)]
    (:response context)))
```

### 16.3 Pureza da configuração

A lista de interceptors é congelada em `create-connector`. Alterações posteriores na
coleção original não devem mudar o connector criado.

Hot reload exige outro mecanismo e fica fora do primeiro gate.

---

## 17. Roteamento desacoplado

### 17.1 Regra arquitetural

O connector não implementa roteamento. Ele executa um dispatch ou uma cadeia.

### 17.2 Roteador mínimo opcional

`cljn.pedestal.route` pode oferecer um primeiro roteador estático:

```clojure
(def routes
  #{["/greet" :get greet-handler :route-name :greet]})

(route/router routes)
```

O resultado é um interceptor comum.

### 17.3 Subconjunto inicial de rotas

- path literal;
- método HTTP;
- handler;
- `:route-name`;
- path params simples `:id`;
- correspondência determinística;
- erro de ambiguidade no build quando detectável.

Ficam fora inicialmente:

- wildcard recursivo;
- constraints por host;
- query constraints;
- expansão dinâmica;
- route fragments definidos por protocols externos;
- live routing.

### 17.4 Compilação da tabela

Rotas estáticas devem ser compiladas em estrutura imutável. O primeiro backend pode usar:

- árvore por segmentos; ou
- tabela ordenada com busca linear para o primeiro corte.

A escolha final deve ser medida. A semântica precede a otimização.

---

## 18. ABI do backend HTTP

### 18.1 Regra

A aplicação Clojure não recebe ponteiros, sockets, `FILE *` ou file descriptors.

### 18.2 Handles opacos

O runtime representa servidor e conexão por handles opacos registrados como recursos
externos, seguindo o modelo de lifecycle da `IO_SPEC`.

### 18.3 Operações mínimas da ABI

Os nomes abaixo são ilustrativos. A ADR-0013 fixa uma ABI dirigida por Clojure: o
provider não retém nem chama uma closure da aplicação.

```c
cljn_http_server_open(config, out_server, out_error)
cljn_http_server_receive(server, out_request, out_error)
cljn_http_server_respond(server, response, out_error)
cljn_http_server_reject(server, error, out_error)
cljn_http_server_stop(server, out_error)
cljn_http_server_close(server)
cljn_http_server_address(server, out_address, out_error)
cljn_http_server_stats(server, out_stats, out_error)
```

`cljn.pedestal.connector/serve!` controla o loop. Uma conexão fica interna ao handle e é
consumida por `respond` ou `reject`, coerente com o primeiro gate single-thread.

### 18.4 Fronteira de dispatch

Não há callback C→Clojure no P1. O provider devolve um request map rooteado; a camada
Clojure executa dispatch e devolve a resposta ao provider. Toda alocação feita durante a
montagem do request deve seguir o protocolo explícito de rooting da ABI.

### 18.5 Erros da ABI

A ABI devolve códigos internos estáveis, convertidos pela camada Clojure em exceções com:

```clojure
{:kind :address-in-use
 :operation :bind
 :host "127.0.0.1"
 :port 8080
 :os-code 98
 :message "Address already in use"}
```

A mensagem e o código do SO são diagnósticos. `:kind` e `:operation` são o contrato.

---

## 19. Provider HTTP inicial

### 19.1 Requisitos

O provider inicial deve:

- suportar Linux x86_64;
- abrir socket TCP IPv4;
- aceitar conexões;
- aplicar timeouts;
- ler request line e headers incrementalmente;
- validar framing antes de alocar body;
- limitar bytes e quantidade de headers;
- ler body por `Content-Length`;
- executar um request por vez;
- serializar resposta HTTP/1.1;
- fechar conexão de forma determinística;
- não vazar recursos em erro;
- funcionar sob ASan/UBSan quando aplicável.

### 19.2 Keep-alive

O primeiro corte pode escolher uma destas opções:

- **MVP-A:** sempre enviar `Connection: close`;
- **MVP-B:** keep-alive sequencial com limite de requests por conexão.

A decisão inicial recomendada é **MVP-A**, reduzindo estado e superfície de segurança.
Keep-alive deve ser um marco posterior com benchmarks próprios.

### 19.3 Parser

O P1 usa um parser C próprio, incremental, estrito e limitado, versionado no subsistema
HTTP do runtime. Ele suporta somente request line, headers e body por
`Content-Length` necessários ao primeiro gate. A adoção de biblioteca externa ou
provider Rust exige nova ADR com licença, fuzzing, tamanho do binário e ownership.

### 19.4 TLS

TLS não deve ser implementado no primeiro provider. Implantação inicial usa proxy reverso
ou load balancer para terminação TLS.

---

## 20. Segurança HTTP mínima

### 20.1 Framing

O provider deve rejeitar:

- `Transfer-Encoding` junto com `Content-Length`;
- múltiplos `Content-Length` divergentes;
- request line acima do limite;
- headers acima do limite total;
- quantidade de headers acima do limite;
- nomes de headers inválidos;
- valores contendo NUL;
- bare CR ou LF inválido;
- chunked encoding no primeiro gate;
- HTTP version desconhecida;
- body maior que o limite.

### 20.2 Request smuggling

A normalização deve ser estrita e não aceitar framing ambíguo. O parser não deve tentar
“ser tolerante” em casos que proxies e servidores possam interpretar de formas diferentes.

### 20.3 Limites padrão

Os valores abaixo são iniciais e devem ser medidos:

```clojure
{:request-line-bytes 8192
 :header-bytes 32768
 :header-count 100
 :body-bytes 1048576
 :requests-per-connection 1}
```

### 20.4 Timeouts

```clojure
{:read-ms 5000
 :write-ms 5000
 :idle-ms 15000}
```

Timeouts devem produzir eventos e fechar a conexão. Nenhum timeout deve ser representado
como retorno parcial válido para a aplicação.

### 20.5 Informações de erro

Respostas 400, 408, 413 e 431 enviadas pelo provider não incluem detalhes de parser,
stack trace, paths ou configuração interna.

---

## 21. Modelo de execução e concorrência

### 21.1 Primeiro gate

```clojure
:mode :single-thread
```

Características:

- um listener;
- no máximo um servidor de rede aberto por processo;
- uma conexão processada por vez;
- nenhuma execução concorrente de Clojure;
- GC somente no thread do servidor;
- comportamento determinístico para testes.

### 21.2 Evolução recomendada

#### Fase futura A — prefork

Múltiplos processos single-threaded, possivelmente com `SO_REUSEPORT` ou listener herdado.
Isso aumenta paralelismo sem tornar o GC thread-safe.

#### Fase futura B — thread pool

Bloqueada por:

- GC thread-safe ou heaps isolados;
- roots por thread;
- Vars dinâmicas por thread;
- exceções e runtime reentrantes;
- auditoria de todas as estruturas globais.

#### Fase futura C — event loop assíncrono

Bloqueada por uma abstração de continuations/tasks e por contrato de async interceptors.
Não deve ser construída como extensão ad hoc do primeiro loop.

---

## 22. Eventos e observabilidade

### 22.1 Event handler

Quando configurado, `:event-handler` recebe maps síncronos e pequenos:

```clojure
{:event :request-complete
 :execution-id 42
 :method :get
 :uri "/greet"
 :status 200
 :duration-ns 153000
 :request-bytes 92
 :response-bytes 19}
```

### 22.2 Eventos mínimos

- `:connector-starting`;
- `:connector-started`;
- `:connector-stopping`;
- `:connector-stopped`;
- `:connection-accepted`;
- `:request-started`;
- `:request-complete`;
- `:request-rejected`;
- `:interceptor-error`;
- `:invalid-response`;
- `:connection-closed`;
- `:connector-failed`.

### 22.3 Falha no event handler

Uma exceção do event handler:

- não altera a resposta da aplicação;
- incrementa contador `:event-handler-errors`;
- é enviada ao stderr por fallback mínimo;
- não é chamada recursivamente como novo evento.

### 22.4 Estatísticas

`connector-stats` retorna snapshot:

```clojure
{:connections-accepted 10
 :connections-active 0
 :requests-total 10
 :responses-2xx 9
 :responses-4xx 1
 :responses-5xx 0
 :request-bytes 910
 :response-bytes 1210
 :parse-errors 1
 :application-errors 0
 :event-handler-errors 0}
```

Contadores podem saturar no maior fixnum em vez de overflow silencioso.

---

## 23. Tratamento de erros

### 23.1 Categorias mínimas

| Categoria | Operação típica |
| --- | --- |
| `:invalid-config` | create |
| `:address-in-use` | bind |
| `:permission-denied` | bind |
| `:already-started` | start |
| `:server-limit` | segundo listener no mesmo processo |
| `:not-running` | operação que exige servidor ativo |
| `:parse-error` | parse request |
| `:request-too-large` | read body |
| `:timeout` | read/write |
| `:connection-reset` | read/write |
| `:invalid-request-map` | test-request |
| `:invalid-response` | serialize response |
| `:application-error` | dispatch/interceptor |
| `:unsupported-feature` | async, chunked, streaming |
| `:internal` | invariantes quebradas |

### 23.2 Erro da aplicação

Erro não tratado por interceptor gera:

- evento `:application-error`;
- resposta 500;
- fechamento da conexão;
- incremento de `:responses-5xx` e `:application-errors`.

### 23.3 Fatalidade

Falhas de invariantes do runtime podem encerrar o processo somente quando a memória ou o
estado do servidor não puderem ser considerados seguros. Erros de input remoto nunca
devem abortar o processo.

---

## 24. `test-request`

### 24.1 Finalidade

Permitir testes rápidos de handlers e interceptors sem:

- bind de porta;
- sockets;
- sleeps;
- concorrência;
- dependência do provider HTTP.

### 24.2 Entrada

Request map mínimo:

```clojure
{:request-method :get
 :uri "/greet"
 :headers {"host" "localhost"}
 :body nil}
```

Campos ausentes recebem defaults documentados:

```clojure
{:server-port 80
 :server-name "localhost"
 :remote-addr "127.0.0.1"
 :query-string nil
 :scheme :http
 :protocol "HTTP/1.1"}
```

### 24.3 Saída

Response map normalizado, sem serialização:

```clojure
{:status 200
 :headers {"content-type" "text/plain; charset=utf-8"
           "content-length" "6"}
 :body "Hello\n"}
```

### 24.4 Erros

`test-request` não converte request map inválido em resposta 400. Ele lança erro
categorizado para facilitar testes. Erros da aplicação seguem a mesma política do
connector e resultam em response 500, salvo opção interna de teste futuro para rethrow.

---

## 25. API de exemplo

### 25.1 Hello World por dispatch direto

```clojure
(ns hello
  (:require [cljn.pedestal.connector :as conn]))

(defn greet [request]
  {:status 200
   :headers {"content-type" "text/plain; charset=utf-8"}
   :body "Hello from native Clojure\n"})

(defn -main [& _]
  (-> (conn/default-connector-map "0.0.0.0" 8080)
      (conn/with-dispatch greet)
      (assoc :join? true)
      conn/create-connector
      conn/start!))
```

### 25.2 Hello World com interceptors e rota

```clojure
(ns hello
  (:require [cljn.pedestal.connector :as conn]
            [cljn.pedestal.route :as route]))

(defn greet-handler [_request]
  {:status 200
   :headers {"content-type" "text/plain; charset=utf-8"}
   :body "Hello, world!\n"})

(def routes
  #{["/greet" :get greet-handler :route-name :greet]})

(defn -main [& _]
  (-> (conn/default-connector-map "0.0.0.0" 8080)
      (conn/with-interceptors [(route/router routes)])
      (assoc :join? true)
      conn/create-connector
      conn/start!))
```

### 25.3 Resultado esperado

```text
$ clojure-native build --source-path examples/pedestal-native/src \
    --main hello examples/pedestal-native/src/hello.clj \
    -o target/pedestal-native
$ ./target/pedestal-native
$ curl -i http://127.0.0.1:8080/greet
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 14
connection: close

Hello, world!
```

---

## 26. Organização proposta do código

A estrutura abaixo é indicativa:

```text
crates/
├── clojure-native-cli/
│   └── src/loader/           # grafo estático, source paths e módulos embutidos
├── clojure-test-support/     # fixtures de projeto/serviço e comparação
└── clojure-codegen/
    └── runtime/
        └── http/
            ├── server.c
            ├── parser.c
            ├── response.c
            └── error.c

stdlib/
└── cljn/
    ├── http/
    │   ├── request.clj
    │   ├── response.clj
    │   └── runtime.clj
    └── pedestal/
        ├── chain.clj
        ├── connector.clj
        └── route.clj

examples/
└── pedestal-native-hello/
    ├── deps.edn
    └── src/hello.clj

tests/conformance/level-e-ecosystem/web-frameworks/pedestal/
└── native-connector-hello/
    ├── case.toml
    ├── deps.edn
    ├── src/hello.clj
    ├── request.http
    └── expected-response.edn
```

Os namespaces não entram em `core_compiled.clj`. O loader estático e os globals
inicializados são pré-requisitos de NC-1, conforme a ADR-0013.

---

## 27. Marcos de implementação

Os marcos são ordenados. Um marco não pode ser declarado completo sem os anteriores.

### NC-0 — Contratos e fixtures

**Objetivo:** transformar esta spec em contratos executáveis, ainda sem rede.

Entregas:

- schema do connector map;
- schema de request e response;
- casos de erros de configuração;
- fixtures de Hello World;
- inventário `pending`/`xfail` na conformidade;
- ADR-0013 adotada como decisão do provider, parser, módulos e lifecycle.

Critério de aceite:

- todos os contratos possuem testes Rust ou Clojure;
- nenhuma API é marcada `active`.

### NC-0L — Módulos estáticos e globals

**Objetivo:** compilar os namespaces Clojure do connector sem incorporá-los ao core.

Entregas:

- grafo literal de `ns`/`:require`;
- módulos embutidos e `--source-path` local, sempre offline;
- diagnósticos por arquivo, namespace duplicado e ciclo;
- `def` imutável inicializado uma vez em ordem topológica;
- roots globais permanentes.

Critério de aceite:

- projeto local de dois namespaces e módulo `cljn.*` compilam sob GC stress;
- nenhum arquivo, JAR ou dependência é resolvido pela rede.

### NC-1 — `test-request` com dispatch direto

**Objetivo:** executar request -> dispatch -> response em memória.

Entregas:

- `default-connector-map`;
- `create-connector` sem provider;
- validação de request map;
- validação de response map;
- `test-request`;
- erros categorizados;
- estatísticas básicas.

Critério de aceite:

```clojure
(= {:status 200 ...}
   (conn/test-request connector request))
```

passa em modo normal e `CLJN_GC_STRESS=1`.

### NC-2 — Chain síncrona de interceptors

**Objetivo:** preservar `:enter`, `:leave`, `:error` e terminação.

Entregas:

- interceptor map/record;
- queue e stack;
- execution id;
- error propagation;
- response terminator;
- observer/event bridge;
- comparação diferencial do subconjunto com Pedestal/JVM.

Critério de aceite:

- corpus cobre ordem, short-circuit, unwind e recuperação de erro;
- resultados observáveis coincidem com o oracle no subconjunto.

### NC-3 — Provider HTTP mínimo

**Objetivo:** responder uma requisição HTTP real.

Entregas:

- socket IPv4;
- bind/listen/accept;
- parser HTTP/1.1 limitado;
- body por `Content-Length`;
- serializer;
- `Connection: close`;
- timeouts;
- lifecycle e sinais;
- leak checks.

Critério de aceite:

```text
curl /greet -> 200 + body esperado
```

em 1.000 execuções consecutivas sem leak detectável.

### NC-4 — Roteador estático

**Objetivo:** executar o Hello World Pedestal-compatible versionado no repositório.

Entregas:

- rotas literais;
- métodos;
- route names;
- path params;
- 404 e 405;
- integração como interceptor.

Critério de aceite:

- fixture `e.pedestal.native_connector_hello` chega a `active`;
- fixture upstream `e.pedestal.hello_world_api` permanece `pending` para P3;
- caso compatível nativo passa offline sem JVM.

### NC-5 — Empacotamento de projetos e source dependencies

**Objetivo:** ampliar o loader estático do P1 para um contrato de projeto reutilizável.

Entregas:

- resolução estática de namespaces;
- ordem de compilação;
- empacotamento de recursos necessários;
- manifesto de source deps;
- diagnóstico de ciclo;
- build reproduzível offline.

Critério de aceite:

- manifesto local reproduzível empacota app e recursos além dos módulos embutidos;
- checksum do build é reproduzível sob ambiente controlado.

### NC-6 — Hardening e benchmark

**Objetivo:** produzir evidência técnica publicável.

Entregas:

- fuzzing do parser;
- ASan/UBSan;
- corpus de smuggling;
- benchmark de startup;
- RSS idle e sob carga;
- tamanho de binário;
- throughput;
- latência p50, p95 e p99;
- CPU por requisição;
- comparação com Pedestal + HTTP-Kit/JVM;
- comparação com alternativa nativa disponível e reproduzível.

Critério de aceite:

- metodologia versionada;
- checksums de respostas equivalentes;
- regressões possuem limites bloqueantes.

### NC-7 — Compatibilidade upstream selecionada

**Objetivo:** avaliar compilação de módulos reais do Pedestal.

Entregas:

- inventário por namespace;
- patches ou adapters necessários;
- versão upstream pinada;
- implementação direta do protocolo de connector, se viável;
- proposta técnica para discussão com mantenedores.

Critério de aceite:

- nenhuma afirmação ampla de compatibilidade;
- matriz pública lista cada namespace e divergência.

---

## 28. Dependências bloqueantes

| Dependência | Marco afetado | Situação |
| --- | --- | --- |
| Erros categorizados | NC-1/NC-2/NC-3 | `throw` de Values existe; P1 padroniza maps, tipos/`ex-data` ficam para P2/P3 |
| Records e protocols | NC-1/NC-2 | disponíveis no subconjunto atual |
| Mapas, vetores, strings e keywords | todos | disponíveis |
| Bytes | NC-1/NC-3 | disponível como `T_BYTES`; faltam contratos HTTP |
| Handles externos | NC-3 | readers/writers existem; falta `T_HTTP_SERVER` e leak gate |
| Sinais/lifecycle de processo | NC-3 | futuro |
| Multi-arquivo estático | NC-0L | ausente |
| Source dependency resolution geral | NC-5/NC-7 | ausente; Maven/JAR não faz parte de P1 |
| Vars dinâmicas | compatibilidade avançada | conhecidas disponíveis; mapa geral de Vars ausente |
| Concorrência thread-safe | modos concorrentes | fora do primeiro gate |

NC-0 pode começar imediatamente. NC-0L deve preceder NC-1 e NC-2, porque o conector
será distribuído como namespaces Clojure estáticos, não como novos built-ins no
compilador. NC-3 requer apenas o subconjunto de handles e erros necessário para sockets;
essa relação é fechada pela ADR-0013 para evitar duplicação incompatível com a
`IO_SPEC`.

---

## 29. Estratégia de testes

### 29.1 Testes unitários

- validação do connector map;
- normalização de headers;
- cálculo de content length;
- lifecycle state machine;
- categorias de erro;
- ordem de interceptors;
- terminação;
- tratamento de response inválida.

### 29.2 Testes de integração

- bind em porta efêmera;
- request HTTP cru;
- body UTF-8 e binário;
- timeout;
- conexão resetada;
- sinal de parada;
- restart do connector;
- 1.000 ciclos start/stop;
- portas ocupadas;
- limites de headers e body.

### 29.3 Testes diferenciais

Para o subconjunto comum, executar os mesmos casos em:

1. Pedestal/JVM com connector HTTP-Kit;
2. `cljn.pedestal.chain` + connector nativo.

Comparar:

- status;
- headers semanticamente relevantes;
- body;
- ordem de eventos de interceptors;
- contexto final sanitizado;
- categoria de erro.

Não comparar:

- mensagens exatas de exceção;
- ordem de headers;
- header `Date`;
- header `Server`;
- IDs de execução;
- detalhes de classe JVM.

### 29.4 Fuzzing

Alvos mínimos:

- request line;
- header parser;
- `Content-Length`;
- percent encoding;
- serializer de headers;
- transição de estados do lifecycle.

Invariantes:

- input remoto não aborta processo;
- nenhuma leitura fora do buffer;
- nenhum tamanho negativo ou overflow;
- nenhum header com CR/LF chega à resposta;
- todo handle é fechado.

### 29.5 GC stress

Todos os caminhos de dispatch, erro e serialização devem rodar com `CLJN_GC_STRESS=1`.
Requests, contexts, interceptors e responses devem permanecer rooted durante toda
travessia da ABI e execução do dispatch.

---

## 30. Benchmark

### 30.1 Aplicações comparáveis

- `GET /plaintext`;
- `GET /json` quando houver encoder comum;
- route param simples;
- cadeia com 1, 5 e 10 interceptors;
- resposta 404;
- erro tratado por interceptor.

### 30.2 Métricas

- tempo de build;
- tamanho do executável stripped e não stripped;
- cold start até aceitar conexão;
- RSS idle;
- RSS após N requests;
- CPU total;
- requests por segundo;
- latência p50, p95 e p99;
- bytes alocados por request, quando mensurável;
- pausas de GC;
- taxa de erro.

### 30.3 Regras metodológicas

- mesma máquina;
- pinagem de CPU quando possível;
- warmup declarado;
- payloads idênticos;
- checksums das respostas;
- pelo menos 5 repetições;
- mediana e dispersão;
- nenhuma comparação de throughput enquanto o modo nativo for estritamente
  single-thread sem deixar essa diferença explícita.

### 30.4 Critério de sucesso inicial

O primeiro gate não exige vencer a JVM em throughput. Ele exige demonstrar:

- startup significativamente menor;
- RSS significativamente menor;
- resposta correta;
- latência estável em carga compatível com o modo single-thread;
- ausência de leaks e crashes.

---

## 31. Compatibilidade declarada

Tabela inicial:

| Recurso | Primeiro gate | Política |
| --- | --- | --- |
| Connector map | sim | API `cljn.*` |
| `start!`/`stop!` | sim | compatível por comportamento |
| `test-request` | sim | compatível por comportamento |
| Request map Ring básico | sim | subconjunto documentado |
| Response map básico | sim | subconjunto documentado |
| Interceptors síncronos | sim | semântica diferencial |
| `:enter`/`:leave`/`:error` | sim | semântica diferencial |
| Terminação por response | sim | interceptor padrão |
| Rotas estáticas | sim, NC-4 | módulo separado |
| Dynamic bindings gerais | não | Vars conhecidas existem; `:bindings` gera erro explícito |
| Async interceptors | não | erro explícito |
| SSE | não | futura spec |
| WebSocket | não | futura spec |
| Streaming | não | futura spec |
| Jetty | não | fora de escopo |
| HTTP-Kit | não | fora de escopo |
| TLS | não | proxy externo |
| HTTP/2 | não | futura spec |
| Middleware Ring arbitrário | não | inventário caso a caso |

---

## 32. Riscos

### R1 — Confundir compatibilidade de API com compilação upstream

**Impacto:** alto.

**Mitigação:** usar labels P0–P3, versão pinada e matriz por recurso.

### R2 — Criar rede antes de estabilizar erros e handles

**Impacto:** leaks, crashes e APIs duplicadas.

**Mitigação:** integrar o lifecycle de sockets ao modelo da `IO_SPEC` e exigir ADR.

### R3 — Parser HTTP inseguro

**Impacto:** request smuggling, DoS e memory safety.

**Mitigação:** parser limitado, fuzzing, corpus adversarial, limites estritos e
`Connection: close` no primeiro gate.

### R4 — Prometer performance baseada em benchmarks algorítmicos

**Impacto:** perda de credibilidade.

**Mitigação:** benchmark HTTP próprio, metodologia pública e comparação justa.

### R5 — Single-thread ser interpretado como arquitetura final

**Impacto:** desenho prematuramente limitado.

**Mitigação:** provider atrás da ABI, estado explícito e roadmap prefork/thread-safe.

### R6 — Duplicar Pedestal em vez de construir um adapter

**Impacto:** manutenção paralela e incompatibilidade crescente.

**Mitigação:** limitar a camada própria ao subconjunto necessário e migrar para módulos
upstream quando o compilador suportar as dependências.

### R7 — Escopo crescer para framework web completo

**Impacto:** atraso no corte vertical.

**Mitigação:** connector não contém router, codecs, sessão, auth ou domínio.

### R8 — Rooting incorreto na fronteira nativa

**Impacto:** corrupção de memória.

**Mitigação:** protocolo de roots na ABI, ausência de callback C→Clojure no P1, GC
stress e testes com alocação na construção do request e em todos os pontos do dispatch.

---

## 33. Decisões fechadas pela ADR-0013

| Tema | Decisão para P1 |
| --- | --- |
| Provider | subsistema C próprio, POSIX/Linux, atrás da ABI atual |
| Parser | C próprio, incremental, estrito e limitado ao framing aceito |
| `Bytes` | reutilizar `T_BYTES`; string exige UTF-8 estrito |
| Sinais | self-pipe não bloqueante + flag `sig_atomic_t`, observado por `poll` |
| Namespace | `cljn.pedestal.*`; nenhum código próprio em `io.pedestal.*` |
| Stdlib | fontes em `stdlib/cljn`, resolvidas por grafo estático offline |
| Roteador | matcher linear determinístico até medição justificar árvore |
| Prefork | fora de P1; exige ADR posterior |
| Oracle | versão fixada pela fixture: Pedestal HTTP-Kit `0.8.2-beta-10` |
| Compatibilidade | fixture P1 nativa separada da fixture upstream P3 |

Threads, async, keep-alive, TLS, HTTP/2, Maven, provider Rust, parser externo e
implementação direta do protocolo upstream continuam decisões futuras e exigem ADR
substituta.

---

## 34. Definition of Done do primeiro corte vertical

O primeiro corte vertical está concluído quando todos os itens abaixo forem verdadeiros:

- [ ] existe um connector map validado;
- [ ] `test-request` executa dispatch direto;
- [ ] a chain síncrona executa `:enter`, `:leave` e `:error`;
- [ ] existe provider HTTP Linux x86_64;
- [ ] `curl /greet` retorna status, headers e body esperados;
- [ ] body de request e response respeita limites;
- [ ] inputs HTTP malformados retornam 400 ou categoria apropriada sem crash;
- [ ] erro da aplicação retorna 500 sem vazar detalhes;
- [ ] `start!`, `stop!` e restart possuem testes;
- [ ] `serve!` e `serve-one!` deixam explícita a ausência de execução em background;
- [ ] `test-request` e rede compartilham o mesmo dispatch;
- [ ] todos os handles são fechados em sucesso e erro;
- [ ] GC stress passa;
- [ ] sanitizers passam no provider;
- [ ] a fixture nativa `e.pedestal.native_connector_hello` está `active`;
- [ ] a fixture upstream `e.pedestal.hello_world_api` permanece classificada como P3;
- [ ] a documentação declara claramente as limitações;
- [ ] existe benchmark inicial de startup, RSS e latência;
- [ ] nenhum resultado é divulgado como “Pedestal completo”.

---

## 35. Primeira sequência de trabalho recomendada

### Etapa 0 — Módulos estáticos

1. implementar grafo literal de namespaces;
2. adicionar módulos `stdlib/cljn` e `--source-path` local;
3. implementar inicialização de `def` imutável e roots globais;
4. validar ciclos, duplicidade, spans por arquivo e GC stress.

### Etapa 1 — Contrato em memória

1. criar tipos de erro do connector;
2. implementar validação do connector map;
3. implementar request/response normalization;
4. implementar `test-request` com dispatch direto;
5. criar fixtures de conformidade.

### Etapa 2 — Interceptors

1. implementar interceptor constructor/predicate;
2. implementar queue/stack síncrona;
3. implementar `:enter` e `:leave`;
4. implementar `:error` e erro não tratado;
5. adicionar terminador por response;
6. construir corpus diferencial contra Pedestal/JVM.

### Etapa 3 — Rede mínima

1. aplicar a ADR-0013 ao provider/parser;
2. implementar handle de servidor;
3. implementar bind/listen/accept;
4. implementar request line e headers;
5. integrar body limitado;
6. serializar resposta;
7. integrar lifecycle, sinais e estatísticas.

### Etapa 4 — Vertical slice Pedestal-compatible

1. implementar roteador literal;
2. implementar path params;
3. montar exemplo `pedestal-native-hello`;
4. ativar caso de conformidade;
5. publicar resultados de startup/RSS.

---

## 36. Critérios para discussão com o time do Pedestal

A proposta só deve ser apresentada como integração concreta quando houver:

1. repositório ou diretório reproduzível;
2. Hello World executável;
3. matriz clara de suporte;
4. testes diferenciais do interceptor chain;
5. benchmark HTTP reproduzível;
6. explicação de que Jetty e HTTP-Kit não são portados;
7. proposta centrada na abstração de connector;
8. lista pequena de pontos em que feedback upstream seria útil.

A pergunta recomendada aos mantenedores não é “vocês adotariam este compilador?”, mas:

> O contrato de connector e o subconjunto síncrono de interceptors definidos aqui
> preservam as invariantes que o Pedestal espera de um adaptador de rede, e quais pontos
> deveriam ser mantidos estáveis para permitir um backend nativo externo?

---

## 37. Resultado esperado da fase inicial

Ao final de NC-4, o projeto deve demonstrar o seguinte fluxo:

```text
source Clojure
  -> análise e AOT pelo clojure-native
  -> binário Linux autônomo
  -> listener HTTP/1.1 nativo
  -> request map
  -> cadeia síncrona de interceptors
  -> roteamento estático
  -> handler
  -> response map
  -> resposta HTTP
```

Esse resultado não representa compatibilidade completa com Pedestal. Ele demonstra que o
modelo arquitetural de connectors, contexts, interceptors e maps pode ser executado em um
runtime Clojure nativo sem JVM, com fronteiras explícitas para evolução posterior.
