// Client-side translations for the static GitHub Pages site.
(() => {
  "use strict";

  const languageConfig = {
    pt: { htmlLang: "pt-BR", locale: "pt_BR", code: "PT", name: "Português", flag: "🇧🇷" },
    es: { htmlLang: "es", locale: "es_ES", code: "ES", name: "Español", flag: "🇪🇸" },
    en: { htmlLang: "en", locale: "en_US", code: "EN", name: "English", flag: "🇺🇸" },
    fr: { htmlLang: "fr", locale: "fr_FR", code: "FR", name: "Français", flag: "🇫🇷" },
    ru: { htmlLang: "ru", locale: "ru_RU", code: "RU", name: "Русский", flag: "🇷🇺" },
    ch: { htmlLang: "zh-CN", locale: "zh_CN", code: "中文", name: "中文", flag: "🇨🇳" },
  };

  const messages = {
    pt: {
      "meta.title": "clojure-native — Clojure compilado. Sem JVM.",
      "meta.description":
        "clojure-native compila um subconjunto documentado de Clojure em executáveis nativos autônomos — sem JVM em tempo de execução.",
      "a11y.skip": "Pular para o conteúdo",
      "a11y.mainNav": "Navegação principal",
      "a11y.footerNav": "Links do rodapé",
      "a11y.menuOpen": "Abrir menu",
      "a11y.menuClose": "Fechar menu",
      "a11y.home": "clojure-native — início",
      "a11y.heroVisual": "Exemplo de compilação de Clojure para um executável nativo",
      "a11y.proof": "Tecnologias e características principais",
      "a11y.requirements": "Requisitos",
      "language.label": "Idioma",
      "nav.project": "O projeto",
      "nav.pipeline": "Pipeline",
      "nav.features": "Recursos",
      "nav.benchmarks": "Benchmarks",
      "nav.start": "Começar",
      "nav.github": "GitHub",
      "hero.eyebrow": "Compilação nativa experimental",
      "hero.titleHtml": "Clojure, <span class=\"accent\">compilado.</span> Sem JVM.",
      "hero.descriptionHtml":
        "<strong>clojure-native</strong> transforma um subconjunto documentado de Clojure em executáveis nativos autônomos, usando Rust, Cranelift e um runtime C compacto.",
      "hero.primaryCta": "Compilar o primeiro programa",
      "hero.source": "Ver código-fonte",
      "hero.codeComment": ";; código Clojure",
      "hero.status": "executável pronto · JVM 0%",
      "proof.noJvm": "Sem JVM em runtime",
      "proof.cranelift": "Codegen Cranelift",
      "proof.gc": "GC preciso",
      "proof.open": "Open source",
      "project.kicker": "Por que existe",
      "project.quoteHtml":
        "A expressividade de Clojure, explorando um caminho <span>inteiramente nativo.</span>",
      "project.binaryTitle": "Binário autônomo",
      "project.binaryBody":
        "O objeto gerado pelo Cranelift é linkado ao runtime C embutido. O programa final não depende da JVM para executar.",
      "project.scopeTitle": "Escopo explícito",
      "project.scopeBody":
        "Compatibilidade, limites e decisões arquiteturais são documentados como parte do produto — sem prometer uma implementação completa.",
      "project.runtimeTitle": "Runtime enxuto",
      "project.runtimeBody":
        "Valores tagueados, coleções persistentes e GC mark-sweep preciso em um runtime C compacto, single-thread e não móvel.",
      "project.labTitle": "Laboratório aberto",
      "project.labBody":
        "Implementação, testes de conformidade, benchmarks e ADRs vivem no mesmo repositório para tornar a evolução verificável.",
      "pipeline.kicker": "Do texto à máquina",
      "pipeline.title": "Um pipeline pequeno. Cada etapa visível.",
      "pipeline.lead":
        "A CLI reúne leitura, interpretação e compilação antecipada. No caminho nativo, o código atravessa quatro camadas bem definidas.",
      "pipeline.input": "Entrada",
      "pipeline.readerBody":
        "Transforma o fonte em forms com spans, macros de leitura, Unicode e diagnósticos determinísticos.",
      "pipeline.semantics": "Semântica",
      "pipeline.analyzerBody":
        "Expande as macros de core suportadas e resolve funções, closures, records, protocolos e controle de fluxo.",
      "pipeline.code": "Código",
      "pipeline.craneliftBody":
        "Emite código de máquina com fast paths nativos para fixnums e acesso direto à shadow stack de roots.",
      "pipeline.output": "Saída",
      "pipeline.linkerTitle": "Linker nativo",
      "pipeline.linkerBody":
        "Combina o objeto gerado com o runtime C e entrega um executável para a plataforma host.",
      "features.kicker": "O que já funciona",
      "features.title": "Mais do que um experimento de “hello world”.",
      "features.lead":
        "Um recorte crescente da linguagem, acompanhado por testes executáveis e documentação de compatibilidade.",
      "features.memoryTag": "Memória gerenciada",
      "features.memoryTitle": "GC preciso com shadow stack de roots",
      "features.memoryBody":
        "Mark-sweep não móvel e single-thread, com loads e stores de roots emitidos diretamente no código gerado.",
      "features.collectionsTag": "Dados imutáveis",
      "features.collectionsTitle": "Coleções persistentes de verdade",
      "features.collectionsBody":
        "Vetores em trie, mapas e sets híbridos com HAMT, estruturas ordenadas em árvore rubro-negra e operações transientes.",
      "features.languageTag": "Linguagem",
      "features.functionsTitle": "Funções & closures",
      "features.functionsBody":
        "Ordem superior, aridades fixas, múltiplas e variádicas, recursão, <code>apply</code> e <code>loop/recur</code>.",
      "features.abstractionsTag": "Abstrações",
      "features.protocolsTitle": "Protocols & records",
      "features.protocolsBody":
        "<code>defrecord</code>, <code>defprotocol</code>, <code>extend-type</code> e dispatch sobre tipos embutidos.",
      "features.coreTitle": "Core compilado",
      "features.coreBody":
        "26 funções, incluindo <code>map</code>, <code>filter</code>, <code>reduce</code>, <code>range</code>, <code>into</code> e <code>comp</code>.",
      "benchmarks.kicker": "Desempenho verificável",
      "benchmarks.title": "Nativo e JVM, caso a caso.",
      "benchmarks.lead":
        "Compare os 90 programas com a mesma carga e checksums idênticos. Escolha a suíte e alterne entre tempo, CPU e memória sem sair da página.",
      "benchmarks.suiteLabel": "Suíte",
      "benchmarks.metricLabel": "Métrica",
      "benchmarks.suiteTabs": "Selecionar suíte",
      "benchmarks.metricTabs": "Selecionar métrica",
      "benchmarks.cormen": "Cormen / CLRS",
      "benchmarks.cracking": "Cracking",
      "benchmarks.wall": "Tempo",
      "benchmarks.cpu": "CPU",
      "benchmarks.memory": "Memória",
      "benchmarks.summaryWall": "tempo total · nativo / JVM",
      "benchmarks.summaryCpu": "CPU total · nativo / JVM",
      "benchmarks.summaryMemory": "RSS mediano · nativo / JVM",
      "benchmarks.altWall":
        "Comparação de tempo da suíte {suite} entre o executável nativo e Clojure/JVM",
      "benchmarks.altCpu":
        "Comparação de CPU da suíte {suite} entre o executável nativo e Clojure/JVM",
      "benchmarks.altMemory":
        "Comparação de memória da suíte {suite} entre o executável nativo e Clojure/JVM",
      "benchmarks.captionWall":
        "Verde favorece o nativo; laranja favorece a JVM. A linha central marca a paridade e a escala é logarítmica.",
      "benchmarks.captionCpu":
        "A razão de CPU total usa a mesma linha de paridade: direita favorece o nativo, esquerda favorece a JVM.",
      "benchmarks.captionMemory":
        "Azul representa o nativo e roxo representa Clojure/JVM. Os pontos mostram RSS absoluto em MiB, em escala logarítmica.",
      "benchmarks.openReport": "Abrir relatório completo",
      "benchmarks.openCsv": "Ver CSV",
      "honesty.label": "Experimental por design",
      "honesty.bodyHtml":
        "<strong>Este projeto ainda não é um substituto para Clojure/JVM.</strong> Interop Java, bignums, sequências lazy, namespaces dinâmicos e projetos multi-arquivo continuam fora do caminho nativo atual.",
      "honesty.status": "v0.0.1 · em evolução",
      "start.kicker": "Comece em minutos",
      "start.title": "Do clone ao binário nativo.",
      "start.lead":
        "Compile a CLI em modo release e transforme qualquer exemplo compatível em um executável autônomo para a sua plataforma host.",
      "start.cCompiler": "Compilador C",
      "code.terminal": "terminal",
      "code.copy": "copiar",
      "code.copied": "copiado",
      "code.select": "selecione o código",
      "code.step1": "# 1. Obtenha o projeto",
      "code.step2": "# 2. Compile a CLI",
      "code.step3": "# 3. Gere e execute um binário",
      "stats.kicker": "Evolução mensurável",
      "stats.title": "Compatibilidade testada, não presumida.",
      "stats.lead":
        "A matriz de conformidade registra recursos entregues, lacunas conhecidas e itens ainda em inventário — tudo verificável offline.",
      "stats.cases": "casos na matriz A–E",
      "stats.active": "ATIVOS",
      "stats.behaviors": "comportamentos implementados",
      "stats.benchmarks": "benchmarks de algoritmos",
      "stats.jvm": "no runtime do binário",
      "stats.note": "números documentados no repositório · versão 0.0.1",
      "cta.title": "Explore. Compile. Contribua.",
      "cta.body":
        "O código, as especificações, os benchmarks e as decisões arquiteturais estão abertos. Veja o estado atual e ajude a construir o próximo passo.",
      "cta.button": "Abrir no GitHub",
      "footer.note": "Projeto experimental sob licenças MIT ou Apache-2.0 ·",
      "footer.docs": "Documentação",
      "footer.specs": "Especificações",
      "footer.benchmarks": "Benchmarks",
      "footer.issues": "Issues",
    },

    es: {
      "meta.title": "clojure-native — Clojure compilado. Sin JVM.",
      "meta.description":
        "clojure-native compila un subconjunto documentado de Clojure en ejecutables nativos autónomos, sin JVM en tiempo de ejecución.",
      "a11y.skip": "Saltar al contenido",
      "a11y.mainNav": "Navegación principal",
      "a11y.footerNav": "Enlaces del pie de página",
      "a11y.menuOpen": "Abrir menú",
      "a11y.menuClose": "Cerrar menú",
      "a11y.home": "clojure-native — inicio",
      "a11y.heroVisual": "Ejemplo de compilación de Clojure a un ejecutable nativo",
      "a11y.proof": "Tecnologías y características principales",
      "a11y.requirements": "Requisitos",
      "language.label": "Idioma",
      "nav.project": "El proyecto",
      "nav.pipeline": "Pipeline",
      "nav.features": "Funciones",
      "nav.benchmarks": "Benchmarks",
      "nav.start": "Empezar",
      "nav.github": "GitHub",
      "hero.eyebrow": "Compilación nativa experimental",
      "hero.titleHtml": "Clojure, <span class=\"accent\">compilado.</span> Sin JVM.",
      "hero.descriptionHtml":
        "<strong>clojure-native</strong> transforma un subconjunto documentado de Clojure en ejecutables nativos autónomos con Rust, Cranelift y un runtime C compacto.",
      "hero.primaryCta": "Compilar el primer programa",
      "hero.source": "Ver código fuente",
      "hero.codeComment": ";; código Clojure",
      "hero.status": "ejecutable listo · JVM 0%",
      "proof.noJvm": "Sin JVM en runtime",
      "proof.cranelift": "Codegen Cranelift",
      "proof.gc": "GC preciso",
      "proof.open": "Código abierto",
      "project.kicker": "Por qué existe",
      "project.quoteHtml":
        "La expresividad de Clojure, explorando un camino <span>completamente nativo.</span>",
      "project.binaryTitle": "Binario autónomo",
      "project.binaryBody":
        "El objeto generado por Cranelift se enlaza con el runtime C integrado. El programa final no depende de la JVM para ejecutarse.",
      "project.scopeTitle": "Alcance explícito",
      "project.scopeBody":
        "La compatibilidad, los límites y las decisiones de arquitectura se documentan como parte del producto, sin prometer una implementación completa.",
      "project.runtimeTitle": "Runtime compacto",
      "project.runtimeBody":
        "Valores etiquetados, colecciones persistentes y GC mark-sweep preciso en un runtime C compacto, single-thread y no móvil.",
      "project.labTitle": "Laboratorio abierto",
      "project.labBody":
        "La implementación, las pruebas de conformidad, los benchmarks y los ADR conviven en el mismo repositorio para que la evolución sea verificable.",
      "pipeline.kicker": "Del texto a la máquina",
      "pipeline.title": "Un pipeline pequeño. Cada etapa visible.",
      "pipeline.lead":
        "La CLI reúne lectura, interpretación y compilación anticipada. En la ruta nativa, el código atraviesa cuatro capas bien definidas.",
      "pipeline.input": "Entrada",
      "pipeline.readerBody":
        "Convierte el código fuente en forms con spans, macros de lectura, Unicode y diagnósticos deterministas.",
      "pipeline.semantics": "Semántica",
      "pipeline.analyzerBody":
        "Expande las macros de core compatibles y resuelve funciones, closures, records, protocolos y control de flujo.",
      "pipeline.code": "Código",
      "pipeline.craneliftBody":
        "Emite código máquina con fast paths nativos para fixnums y acceso directo a la shadow stack de roots.",
      "pipeline.output": "Salida",
      "pipeline.linkerTitle": "Linker nativo",
      "pipeline.linkerBody":
        "Combina el objeto generado con el runtime C y entrega un ejecutable para la plataforma host.",
      "features.kicker": "Lo que ya funciona",
      "features.title": "Más que un experimento de “hello world”.",
      "features.lead":
        "Un subconjunto creciente del lenguaje acompañado de pruebas ejecutables y documentación de compatibilidad.",
      "features.memoryTag": "Memoria gestionada",
      "features.memoryTitle": "GC preciso con shadow stack de roots",
      "features.memoryBody":
        "Mark-sweep no móvil y single-thread, con cargas y escrituras de roots emitidas directamente en el código generado.",
      "features.collectionsTag": "Datos inmutables",
      "features.collectionsTitle": "Colecciones realmente persistentes",
      "features.collectionsBody":
        "Vectores en trie, mapas y sets híbridos con HAMT, estructuras ordenadas en árbol rojinegro y operaciones transitorias.",
      "features.languageTag": "Lenguaje",
      "features.functionsTitle": "Funciones y closures",
      "features.functionsBody":
        "Orden superior, aridades fijas, múltiples y variádicas, recursión, <code>apply</code> y <code>loop/recur</code>.",
      "features.abstractionsTag": "Abstracciones",
      "features.protocolsTitle": "Protocols y records",
      "features.protocolsBody":
        "<code>defrecord</code>, <code>defprotocol</code>, <code>extend-type</code> y dispatch sobre tipos integrados.",
      "features.coreTitle": "Core compilado",
      "features.coreBody":
        "26 funciones, entre ellas <code>map</code>, <code>filter</code>, <code>reduce</code>, <code>range</code>, <code>into</code> y <code>comp</code>.",
      "benchmarks.kicker": "Rendimiento verificable",
      "benchmarks.title": "Nativo y JVM, caso por caso.",
      "benchmarks.lead":
        "Compara los 90 programas con la misma carga y checksums idénticos. Elige la suite y alterna entre tiempo, CPU y memoria sin salir de la página.",
      "benchmarks.suiteLabel": "Suite",
      "benchmarks.metricLabel": "Métrica",
      "benchmarks.suiteTabs": "Seleccionar suite",
      "benchmarks.metricTabs": "Seleccionar métrica",
      "benchmarks.cormen": "Cormen / CLRS",
      "benchmarks.cracking": "Cracking",
      "benchmarks.wall": "Tiempo",
      "benchmarks.cpu": "CPU",
      "benchmarks.memory": "Memoria",
      "benchmarks.summaryWall": "tiempo total · nativo / JVM",
      "benchmarks.summaryCpu": "CPU total · nativo / JVM",
      "benchmarks.summaryMemory": "RSS mediano · nativo / JVM",
      "benchmarks.altWall":
        "Comparación de tiempo de la suite {suite} entre el ejecutable nativo y Clojure/JVM",
      "benchmarks.altCpu":
        "Comparación de CPU de la suite {suite} entre el ejecutable nativo y Clojure/JVM",
      "benchmarks.altMemory":
        "Comparación de memoria de la suite {suite} entre el ejecutable nativo y Clojure/JVM",
      "benchmarks.captionWall":
        "El verde favorece al nativo y el naranja a la JVM. La línea central marca la paridad y la escala es logarítmica.",
      "benchmarks.captionCpu":
        "La razón de CPU total usa la misma línea de paridad: la derecha favorece al nativo y la izquierda a la JVM.",
      "benchmarks.captionMemory":
        "Azul representa al nativo y morado a Clojure/JVM. Los puntos muestran RSS absoluto en MiB, en escala logarítmica.",
      "benchmarks.openReport": "Abrir informe completo",
      "benchmarks.openCsv": "Ver CSV",
      "honesty.label": "Experimental por diseño",
      "honesty.bodyHtml":
        "<strong>Este proyecto aún no sustituye a Clojure/JVM.</strong> La interoperabilidad Java, bignums, secuencias lazy, namespaces dinámicos y proyectos multiarchivo siguen fuera de la ruta nativa actual.",
      "honesty.status": "v0.0.1 · en evolución",
      "start.kicker": "Empieza en minutos",
      "start.title": "Del clon al binario nativo.",
      "start.lead":
        "Compila la CLI en modo release y convierte cualquier ejemplo compatible en un ejecutable autónomo para tu plataforma host.",
      "start.cCompiler": "Compilador C",
      "code.terminal": "terminal",
      "code.copy": "copiar",
      "code.copied": "copiado",
      "code.select": "selecciona el código",
      "code.step1": "# 1. Obtén el proyecto",
      "code.step2": "# 2. Compila la CLI",
      "code.step3": "# 3. Genera y ejecuta un binario",
      "stats.kicker": "Evolución medible",
      "stats.title": "Compatibilidad probada, no asumida.",
      "stats.lead":
        "La matriz de conformidad registra funciones entregadas, brechas conocidas y elementos aún en inventario; todo verificable sin conexión.",
      "stats.cases": "casos en la matriz A–E",
      "stats.active": "ACTIVOS",
      "stats.behaviors": "comportamientos implementados",
      "stats.benchmarks": "benchmarks de algoritmos",
      "stats.jvm": "en el runtime del binario",
      "stats.note": "cifras documentadas en el repositorio · versión 0.0.1",
      "cta.title": "Explora. Compila. Contribuye.",
      "cta.body":
        "El código, las especificaciones, los benchmarks y las decisiones de arquitectura son abiertos. Consulta el estado actual y ayuda a construir el siguiente paso.",
      "cta.button": "Abrir en GitHub",
      "footer.note": "Proyecto experimental bajo licencias MIT o Apache-2.0 ·",
      "footer.docs": "Documentación",
      "footer.specs": "Especificaciones",
      "footer.benchmarks": "Benchmarks",
      "footer.issues": "Issues",
    },

    en: {
      "meta.title": "clojure-native — Clojure compiled. No JVM.",
      "meta.description":
        "clojure-native compiles a documented Clojure subset into standalone native executables, with no JVM at runtime.",
      "a11y.skip": "Skip to content",
      "a11y.mainNav": "Main navigation",
      "a11y.footerNav": "Footer links",
      "a11y.menuOpen": "Open menu",
      "a11y.menuClose": "Close menu",
      "a11y.home": "clojure-native — home",
      "a11y.heroVisual": "Example of compiling Clojure into a native executable",
      "a11y.proof": "Core technologies and features",
      "a11y.requirements": "Requirements",
      "language.label": "Language",
      "nav.project": "The project",
      "nav.pipeline": "Pipeline",
      "nav.features": "Features",
      "nav.benchmarks": "Benchmarks",
      "nav.start": "Get started",
      "nav.github": "GitHub",
      "hero.eyebrow": "Experimental native compilation",
      "hero.titleHtml": "Clojure, <span class=\"accent\">compiled.</span> No JVM.",
      "hero.descriptionHtml":
        "<strong>clojure-native</strong> turns a documented Clojure subset into standalone native executables using Rust, Cranelift, and a compact C runtime.",
      "hero.primaryCta": "Compile your first program",
      "hero.source": "View source code",
      "hero.codeComment": ";; Clojure source",
      "hero.status": "executable ready · JVM 0%",
      "proof.noJvm": "No JVM at runtime",
      "proof.cranelift": "Cranelift codegen",
      "proof.gc": "Precise GC",
      "proof.open": "Open source",
      "project.kicker": "Why it exists",
      "project.quoteHtml":
        "Clojure's expressiveness, exploring a path that is <span>native all the way down.</span>",
      "project.binaryTitle": "Standalone binary",
      "project.binaryBody":
        "The object emitted by Cranelift is linked with the embedded C runtime. The final program does not depend on the JVM to run.",
      "project.scopeTitle": "Explicit scope",
      "project.scopeBody":
        "Compatibility, boundaries, and architecture decisions are documented as part of the product, without promising a complete implementation.",
      "project.runtimeTitle": "Lean runtime",
      "project.runtimeBody":
        "Tagged values, persistent collections, and precise mark-sweep GC in a compact, single-threaded, non-moving C runtime.",
      "project.labTitle": "Open laboratory",
      "project.labBody":
        "Implementation, conformance tests, benchmarks, and ADRs live in one repository so progress remains verifiable.",
      "pipeline.kicker": "From text to machine",
      "pipeline.title": "A small pipeline. Every stage visible.",
      "pipeline.lead":
        "The CLI brings together reading, interpretation, and ahead-of-time compilation. On the native path, code crosses four clearly defined layers.",
      "pipeline.input": "Input",
      "pipeline.readerBody":
        "Turns source into forms with spans, reader macros, Unicode support, and deterministic diagnostics.",
      "pipeline.semantics": "Semantics",
      "pipeline.analyzerBody":
        "Expands supported core macros and resolves functions, closures, records, protocols, and control flow.",
      "pipeline.code": "Code",
      "pipeline.craneliftBody":
        "Emits machine code with native fast paths for fixnums and direct access to the root shadow stack.",
      "pipeline.output": "Output",
      "pipeline.linkerTitle": "Native linker",
      "pipeline.linkerBody":
        "Combines the generated object with the C runtime and delivers an executable for the host platform.",
      "features.kicker": "What works today",
      "features.title": "More than a “hello world” experiment.",
      "features.lead":
        "A growing language subset backed by executable tests and compatibility documentation.",
      "features.memoryTag": "Managed memory",
      "features.memoryTitle": "Precise GC with a root shadow stack",
      "features.memoryBody":
        "Single-threaded, non-moving mark-sweep with root loads and stores emitted directly into generated code.",
      "features.collectionsTag": "Immutable data",
      "features.collectionsTitle": "Truly persistent collections",
      "features.collectionsBody":
        "Trie vectors, hybrid maps and sets with HAMT, ordered red-black-tree structures, and transient operations.",
      "features.languageTag": "Language",
      "features.functionsTitle": "Functions & closures",
      "features.functionsBody":
        "Higher order, fixed, multiple, and variadic arities, recursion, <code>apply</code>, and <code>loop/recur</code>.",
      "features.abstractionsTag": "Abstractions",
      "features.protocolsTitle": "Protocols & records",
      "features.protocolsBody":
        "<code>defrecord</code>, <code>defprotocol</code>, <code>extend-type</code>, and dispatch over built-in types.",
      "features.coreTitle": "Compiled core",
      "features.coreBody":
        "26 functions including <code>map</code>, <code>filter</code>, <code>reduce</code>, <code>range</code>, <code>into</code>, and <code>comp</code>.",
      "benchmarks.kicker": "Verifiable performance",
      "benchmarks.title": "Native and JVM, case by case.",
      "benchmarks.lead":
        "Compare all 90 programs under the same load with matching checksums. Choose a suite and switch between wall time, CPU, and memory without leaving the page.",
      "benchmarks.suiteLabel": "Suite",
      "benchmarks.metricLabel": "Metric",
      "benchmarks.suiteTabs": "Select benchmark suite",
      "benchmarks.metricTabs": "Select benchmark metric",
      "benchmarks.cormen": "Cormen / CLRS",
      "benchmarks.cracking": "Cracking",
      "benchmarks.wall": "Wall time",
      "benchmarks.cpu": "CPU",
      "benchmarks.memory": "Memory",
      "benchmarks.summaryWall": "total wall time · native / JVM",
      "benchmarks.summaryCpu": "total CPU · native / JVM",
      "benchmarks.summaryMemory": "median RSS · native / JVM",
      "benchmarks.altWall":
        "Wall-time comparison for the {suite} suite between the native executable and Clojure/JVM",
      "benchmarks.altCpu":
        "CPU comparison for the {suite} suite between the native executable and Clojure/JVM",
      "benchmarks.altMemory":
        "Memory comparison for the {suite} suite between the native executable and Clojure/JVM",
      "benchmarks.captionWall":
        "Green favors native; orange favors the JVM. The center line marks parity and the scale is logarithmic.",
      "benchmarks.captionCpu":
        "Total CPU uses the same parity line: right favors native, while left favors the JVM.",
      "benchmarks.captionMemory":
        "Blue represents native and purple represents Clojure/JVM. Points show absolute RSS in MiB on a logarithmic scale.",
      "benchmarks.openReport": "Open full report",
      "benchmarks.openCsv": "View CSV",
      "honesty.label": "Experimental by design",
      "honesty.bodyHtml":
        "<strong>This project is not yet a replacement for Clojure/JVM.</strong> Java interop, bignums, lazy sequences, dynamic namespaces, and multi-file projects remain outside the current native path.",
      "honesty.status": "v0.0.1 · evolving",
      "start.kicker": "Start in minutes",
      "start.title": "From clone to native binary.",
      "start.lead":
        "Build the CLI in release mode and turn any compatible example into a standalone executable for your host platform.",
      "start.cCompiler": "C compiler",
      "code.terminal": "terminal",
      "code.copy": "copy",
      "code.copied": "copied",
      "code.select": "select the code",
      "code.step1": "# 1. Get the project",
      "code.step2": "# 2. Build the CLI",
      "code.step3": "# 3. Build and run a binary",
      "stats.kicker": "Measurable progress",
      "stats.title": "Compatibility tested, not assumed.",
      "stats.lead":
        "The conformance matrix records delivered features, known gaps, and items still under inventory — all verifiable offline.",
      "stats.cases": "cases in the A–E matrix",
      "stats.active": "ACTIVE",
      "stats.behaviors": "implemented behaviors",
      "stats.benchmarks": "algorithm benchmarks",
      "stats.jvm": "in the binary runtime",
      "stats.note": "numbers documented in the repository · version 0.0.1",
      "cta.title": "Explore. Compile. Contribute.",
      "cta.body":
        "The code, specifications, benchmarks, and architecture decisions are open. See the current status and help build the next step.",
      "cta.button": "Open on GitHub",
      "footer.note": "Experimental project under MIT or Apache-2.0 licenses ·",
      "footer.docs": "Documentation",
      "footer.specs": "Specifications",
      "footer.benchmarks": "Benchmarks",
      "footer.issues": "Issues",
    },

    fr: {
      "meta.title": "clojure-native — Clojure compilé. Sans JVM.",
      "meta.description":
        "clojure-native compile un sous-ensemble documenté de Clojure en exécutables natifs autonomes, sans JVM à l'exécution.",
      "a11y.skip": "Aller au contenu",
      "a11y.mainNav": "Navigation principale",
      "a11y.footerNav": "Liens de pied de page",
      "a11y.menuOpen": "Ouvrir le menu",
      "a11y.menuClose": "Fermer le menu",
      "a11y.home": "clojure-native — accueil",
      "a11y.heroVisual": "Exemple de compilation de Clojure en exécutable natif",
      "a11y.proof": "Technologies et caractéristiques principales",
      "a11y.requirements": "Prérequis",
      "language.label": "Langue",
      "nav.project": "Le projet",
      "nav.pipeline": "Pipeline",
      "nav.features": "Fonctions",
      "nav.benchmarks": "Benchmarks",
      "nav.start": "Démarrer",
      "nav.github": "GitHub",
      "hero.eyebrow": "Compilation native expérimentale",
      "hero.titleHtml": "Clojure, <span class=\"accent\">compilé.</span> Sans JVM.",
      "hero.descriptionHtml":
        "<strong>clojure-native</strong> transforme un sous-ensemble documenté de Clojure en exécutables natifs autonomes avec Rust, Cranelift et un runtime C compact.",
      "hero.primaryCta": "Compiler le premier programme",
      "hero.source": "Voir le code source",
      "hero.codeComment": ";; code Clojure",
      "hero.status": "exécutable prêt · JVM 0 %",
      "proof.noJvm": "Sans JVM à l'exécution",
      "proof.cranelift": "Codegen Cranelift",
      "proof.gc": "GC précis",
      "proof.open": "Open source",
      "project.kicker": "Pourquoi il existe",
      "project.quoteHtml":
        "L'expressivité de Clojure, en explorant une voie <span>entièrement native.</span>",
      "project.binaryTitle": "Binaire autonome",
      "project.binaryBody":
        "L'objet généré par Cranelift est lié au runtime C intégré. Le programme final ne dépend pas de la JVM pour s'exécuter.",
      "project.scopeTitle": "Périmètre explicite",
      "project.scopeBody":
        "La compatibilité, les limites et les décisions d'architecture font partie de la documentation du produit, sans promettre une implémentation complète.",
      "project.runtimeTitle": "Runtime compact",
      "project.runtimeBody":
        "Valeurs taguées, collections persistantes et GC mark-sweep précis dans un runtime C compact, mono-thread et non mobile.",
      "project.labTitle": "Laboratoire ouvert",
      "project.labBody":
        "Implémentation, tests de conformité, benchmarks et ADR sont réunis dans le même dépôt pour rendre l'évolution vérifiable.",
      "pipeline.kicker": "Du texte à la machine",
      "pipeline.title": "Un petit pipeline. Chaque étape visible.",
      "pipeline.lead":
        "La CLI réunit lecture, interprétation et compilation anticipée. Sur la voie native, le code traverse quatre couches clairement définies.",
      "pipeline.input": "Entrée",
      "pipeline.readerBody":
        "Transforme la source en forms avec spans, macros de lecture, Unicode et diagnostics déterministes.",
      "pipeline.semantics": "Sémantique",
      "pipeline.analyzerBody":
        "Développe les macros de core prises en charge et résout fonctions, closures, records, protocoles et flux de contrôle.",
      "pipeline.code": "Code",
      "pipeline.craneliftBody":
        "Émet du code machine avec des fast paths natifs pour les fixnums et un accès direct à la shadow stack des roots.",
      "pipeline.output": "Sortie",
      "pipeline.linkerTitle": "Linker natif",
      "pipeline.linkerBody":
        "Combine l'objet généré avec le runtime C et produit un exécutable pour la plateforme hôte.",
      "features.kicker": "Ce qui fonctionne déjà",
      "features.title": "Plus qu'une expérience « hello world ».",
      "features.lead":
        "Un sous-ensemble croissant du langage, accompagné de tests exécutables et d'une documentation de compatibilité.",
      "features.memoryTag": "Mémoire gérée",
      "features.memoryTitle": "GC précis avec shadow stack de roots",
      "features.memoryBody":
        "Mark-sweep non mobile et mono-thread, avec chargements et écritures de roots émis directement dans le code généré.",
      "features.collectionsTag": "Données immuables",
      "features.collectionsTitle": "De vraies collections persistantes",
      "features.collectionsBody":
        "Vecteurs en trie, maps et sets hybrides avec HAMT, structures ordonnées en arbre rouge-noir et opérations transitoires.",
      "features.languageTag": "Langage",
      "features.functionsTitle": "Fonctions et closures",
      "features.functionsBody":
        "Ordre supérieur, arités fixes, multiples et variadiques, récursion, <code>apply</code> et <code>loop/recur</code>.",
      "features.abstractionsTag": "Abstractions",
      "features.protocolsTitle": "Protocols et records",
      "features.protocolsBody":
        "<code>defrecord</code>, <code>defprotocol</code>, <code>extend-type</code> et dispatch sur les types intégrés.",
      "features.coreTitle": "Core compilé",
      "features.coreBody":
        "26 fonctions, dont <code>map</code>, <code>filter</code>, <code>reduce</code>, <code>range</code>, <code>into</code> et <code>comp</code>.",
      "benchmarks.kicker": "Performance vérifiable",
      "benchmarks.title": "Natif et JVM, cas par cas.",
      "benchmarks.lead":
        "Comparez les 90 programmes avec la même charge et des checksums identiques. Choisissez une suite et alternez entre temps d’exécution, CPU et mémoire sans quitter la page.",
      "benchmarks.suiteLabel": "Suite",
      "benchmarks.metricLabel": "Métrique",
      "benchmarks.suiteTabs": "Sélectionner la suite",
      "benchmarks.metricTabs": "Sélectionner la métrique",
      "benchmarks.cormen": "Cormen / CLRS",
      "benchmarks.cracking": "Cracking",
      "benchmarks.wall": "Temps",
      "benchmarks.cpu": "CPU",
      "benchmarks.memory": "Mémoire",
      "benchmarks.summaryWall": "temps total · natif / JVM",
      "benchmarks.summaryCpu": "CPU total · natif / JVM",
      "benchmarks.summaryMemory": "RSS médian · natif / JVM",
      "benchmarks.altWall":
        "Comparaison du temps de la suite {suite} entre l'exécutable natif et Clojure/JVM",
      "benchmarks.altCpu":
        "Comparaison CPU de la suite {suite} entre l'exécutable natif et Clojure/JVM",
      "benchmarks.altMemory":
        "Comparaison mémoire de la suite {suite} entre l'exécutable natif et Clojure/JVM",
      "benchmarks.captionWall":
        "Le vert favorise le natif et l'orange la JVM. La ligne centrale marque la parité et l'échelle est logarithmique.",
      "benchmarks.captionCpu":
        "Le CPU total utilise la même ligne de parité : la droite favorise le natif, la gauche la JVM.",
      "benchmarks.captionMemory":
        "Le bleu représente le natif et le violet Clojure/JVM. Les points indiquent le RSS absolu en MiB sur une échelle logarithmique.",
      "benchmarks.openReport": "Ouvrir le rapport complet",
      "benchmarks.openCsv": "Voir le CSV",
      "honesty.label": "Expérimental par conception",
      "honesty.bodyHtml":
        "<strong>Ce projet ne remplace pas encore Clojure/JVM.</strong> L'interop Java, les bignums, les séquences lazy, les namespaces dynamiques et les projets multi-fichiers restent hors de la voie native actuelle.",
      "honesty.status": "v0.0.1 · en évolution",
      "start.kicker": "Démarrez en quelques minutes",
      "start.title": "Du clone au binaire natif.",
      "start.lead":
        "Compilez la CLI en mode release et transformez tout exemple compatible en exécutable autonome pour votre plateforme hôte.",
      "start.cCompiler": "Compilateur C",
      "code.terminal": "terminal",
      "code.copy": "copier",
      "code.copied": "copié",
      "code.select": "sélectionnez le code",
      "code.step1": "# 1. Récupérez le projet",
      "code.step2": "# 2. Compilez la CLI",
      "code.step3": "# 3. Générez et exécutez un binaire",
      "stats.kicker": "Évolution mesurable",
      "stats.title": "Compatibilité testée, jamais supposée.",
      "stats.lead":
        "La matrice de conformité recense les fonctions livrées, les lacunes connues et les éléments encore en inventaire — le tout vérifiable hors ligne.",
      "stats.cases": "cas dans la matrice A–E",
      "stats.active": "ACTIFS",
      "stats.behaviors": "comportements implémentés",
      "stats.benchmarks": "benchmarks d'algorithmes",
      "stats.jvm": "dans le runtime du binaire",
      "stats.note": "chiffres documentés dans le dépôt · version 0.0.1",
      "cta.title": "Explorez. Compilez. Contribuez.",
      "cta.body":
        "Le code, les spécifications, les benchmarks et les décisions d'architecture sont ouverts. Consultez l'état actuel et contribuez à la prochaine étape.",
      "cta.button": "Ouvrir sur GitHub",
      "footer.note": "Projet expérimental sous licences MIT ou Apache-2.0 ·",
      "footer.docs": "Documentation",
      "footer.specs": "Spécifications",
      "footer.benchmarks": "Benchmarks",
      "footer.issues": "Issues",
    },

    ru: {
      "meta.title": "clojure-native — Clojure с нативной компиляцией. Без JVM.",
      "meta.description":
        "clojure-native компилирует документированное подмножество Clojure в автономные нативные исполняемые файлы без JVM во время выполнения.",
      "a11y.skip": "Перейти к содержимому",
      "a11y.mainNav": "Основная навигация",
      "a11y.footerNav": "Ссылки в подвале",
      "a11y.menuOpen": "Открыть меню",
      "a11y.menuClose": "Закрыть меню",
      "a11y.home": "clojure-native — главная",
      "a11y.heroVisual": "Пример компиляции Clojure в нативный исполняемый файл",
      "a11y.proof": "Основные технологии и возможности",
      "a11y.requirements": "Требования",
      "language.label": "Язык",
      "nav.project": "Проект",
      "nav.pipeline": "Конвейер",
      "nav.features": "Возможности",
      "nav.benchmarks": "Бенчмарки",
      "nav.start": "Начало работы",
      "nav.github": "GitHub",
      "hero.eyebrow": "Экспериментальная нативная компиляция",
      "hero.titleHtml": "Clojure. <span class=\"accent\">Нативно.</span> Без JVM.",
      "hero.descriptionHtml":
        "<strong>clojure-native</strong> превращает документированное подмножество Clojure в автономные нативные исполняемые файлы с помощью Rust, Cranelift и компактного runtime на C.",
      "hero.primaryCta": "Скомпилировать первую программу",
      "hero.source": "Посмотреть исходный код",
      "hero.codeComment": ";; код Clojure",
      "hero.status": "исполняемый файл готов · JVM 0%",
      "proof.noJvm": "Без JVM в runtime",
      "proof.cranelift": "Codegen Cranelift",
      "proof.gc": "Точный GC",
      "proof.open": "Открытый код",
      "project.kicker": "Зачем он нужен",
      "project.quoteHtml":
        "Выразительность Clojure на пути к <span>полностью нативному выполнению.</span>",
      "project.binaryTitle": "Автономный бинарный файл",
      "project.binaryBody":
        "Объект, созданный Cranelift, связывается со встроенным runtime на C. Итоговая программа не зависит от JVM.",
      "project.scopeTitle": "Явные границы",
      "project.scopeBody":
        "Совместимость, ограничения и архитектурные решения документируются как часть продукта без обещания полной реализации.",
      "project.runtimeTitle": "Компактный runtime",
      "project.runtimeBody":
        "Тегированные значения, персистентные коллекции и точный mark-sweep GC в компактном, однопоточном и неперемещающем runtime на C.",
      "project.labTitle": "Открытая лаборатория",
      "project.labBody":
        "Реализация, тесты соответствия, бенчмарки и ADR находятся в одном репозитории, поэтому прогресс можно проверить.",
      "pipeline.kicker": "От текста к машине",
      "pipeline.title": "Небольшой конвейер. Каждый этап виден.",
      "pipeline.lead":
        "CLI объединяет чтение, интерпретацию и опережающую компиляцию. На нативном пути код проходит четыре чётко определённых слоя.",
      "pipeline.input": "Ввод",
      "pipeline.readerBody":
        "Преобразует исходный код в forms со spans, reader-макросами, Unicode и детерминированной диагностикой.",
      "pipeline.semantics": "Семантика",
      "pipeline.analyzerBody":
        "Раскрывает поддерживаемые макросы core и разрешает функции, closures, records, протоколы и поток управления.",
      "pipeline.code": "Код",
      "pipeline.craneliftBody":
        "Генерирует машинный код с нативными fast paths для fixnums и прямым доступом к shadow stack корней.",
      "pipeline.output": "Вывод",
      "pipeline.linkerTitle": "Нативный линкер",
      "pipeline.linkerBody":
        "Объединяет созданный объект с runtime на C и выдаёт исполняемый файл для хост-платформы.",
      "features.kicker": "Что уже работает",
      "features.title": "Больше, чем эксперимент «hello world».",
      "features.lead":
        "Растущее подмножество языка, подкреплённое исполняемыми тестами и документацией совместимости.",
      "features.memoryTag": "Управляемая память",
      "features.memoryTitle": "Точный GC с shadow stack корней",
      "features.memoryBody":
        "Однопоточный неперемещающий mark-sweep с загрузкой и сохранением корней непосредственно в сгенерированном коде.",
      "features.collectionsTag": "Неизменяемые данные",
      "features.collectionsTitle": "Настоящие персистентные коллекции",
      "features.collectionsBody":
        "Векторы на trie, гибридные maps и sets с HAMT, упорядоченные структуры на красно-чёрном дереве и transient-операции.",
      "features.languageTag": "Язык",
      "features.functionsTitle": "Функции и closures",
      "features.functionsBody":
        "Функции высшего порядка, фиксированные, множественные и вариативные арности, рекурсия, <code>apply</code> и <code>loop/recur</code>.",
      "features.abstractionsTag": "Абстракции",
      "features.protocolsTitle": "Protocols и records",
      "features.protocolsBody":
        "<code>defrecord</code>, <code>defprotocol</code>, <code>extend-type</code> и dispatch для встроенных типов.",
      "features.coreTitle": "Скомпилированный core",
      "features.coreBody":
        "26 функций, включая <code>map</code>, <code>filter</code>, <code>reduce</code>, <code>range</code>, <code>into</code> и <code>comp</code>.",
      "benchmarks.kicker": "Проверяемая производительность",
      "benchmarks.title": "Нативный код и JVM — тест за тестом.",
      "benchmarks.lead":
        "Сравните 90 программ с одинаковой нагрузкой и совпадающими контрольными суммами. Выбирайте набор и переключайтесь между временем, CPU и памятью прямо на странице.",
      "benchmarks.suiteLabel": "Набор",
      "benchmarks.metricLabel": "Метрика",
      "benchmarks.suiteTabs": "Выбрать набор тестов",
      "benchmarks.metricTabs": "Выбрать метрику",
      "benchmarks.cormen": "Cormen / CLRS",
      "benchmarks.cracking": "Cracking",
      "benchmarks.wall": "Время",
      "benchmarks.cpu": "CPU",
      "benchmarks.memory": "Память",
      "benchmarks.summaryWall": "общее время · native / JVM",
      "benchmarks.summaryCpu": "общее CPU · native / JVM",
      "benchmarks.summaryMemory": "медианный RSS · native / JVM",
      "benchmarks.altWall":
        "Сравнение времени набора {suite} между нативным исполняемым файлом и Clojure/JVM",
      "benchmarks.altCpu":
        "Сравнение CPU набора {suite} между нативным исполняемым файлом и Clojure/JVM",
      "benchmarks.altMemory":
        "Сравнение памяти набора {suite} между нативным исполняемым файлом и Clojure/JVM",
      "benchmarks.captionWall":
        "Зелёный цвет означает преимущество native, оранжевый — JVM. Центральная линия обозначает паритет; шкала логарифмическая.",
      "benchmarks.captionCpu":
        "Общее CPU использует ту же линию паритета: справа преимущество native, слева — JVM.",
      "benchmarks.captionMemory":
        "Синий обозначает native, фиолетовый — Clojure/JVM. Точки показывают абсолютный RSS в MiB по логарифмической шкале.",
      "benchmarks.openReport": "Открыть полный отчёт",
      "benchmarks.openCsv": "Открыть CSV",
      "honesty.label": "Экспериментальный по замыслу",
      "honesty.bodyHtml":
        "<strong>Проект пока не заменяет Clojure/JVM.</strong> Java interop, bignums, ленивые последовательности, динамические namespaces и многофайловые проекты остаются за пределами текущего нативного пути.",
      "honesty.status": "v0.0.1 · развивается",
      "start.kicker": "Начните за несколько минут",
      "start.title": "От клонирования до нативного бинарного файла.",
      "start.lead":
        "Соберите CLI в режиме release и превратите любой совместимый пример в автономный исполняемый файл для вашей хост-платформы.",
      "start.cCompiler": "Компилятор C",
      "code.terminal": "терминал",
      "code.copy": "копировать",
      "code.copied": "скопировано",
      "code.select": "выделите код",
      "code.step1": "# 1. Получите проект",
      "code.step2": "# 2. Соберите CLI",
      "code.step3": "# 3. Создайте и запустите бинарный файл",
      "stats.kicker": "Измеримый прогресс",
      "stats.title": "Совместимость проверяется, а не предполагается.",
      "stats.lead":
        "Матрица соответствия фиксирует реализованные возможности, известные пробелы и пункты инвентаризации — всё можно проверить офлайн.",
      "stats.cases": "случаев в матрице A–E",
      "stats.active": "АКТИВНЫ",
      "stats.behaviors": "реализованных поведений",
      "stats.benchmarks": "алгоритмических бенчмарков",
      "stats.jvm": "в runtime бинарного файла",
      "stats.note": "цифры задокументированы в репозитории · версия 0.0.1",
      "cta.title": "Изучайте. Компилируйте. Участвуйте.",
      "cta.body":
        "Код, спецификации, бенчмарки и архитектурные решения открыты. Ознакомьтесь с текущим состоянием и помогите сделать следующий шаг.",
      "cta.button": "Открыть на GitHub",
      "footer.note": "Экспериментальный проект под лицензиями MIT или Apache-2.0 ·",
      "footer.docs": "Документация",
      "footer.specs": "Спецификации",
      "footer.benchmarks": "Бенчмарки",
      "footer.issues": "Issues",
    },

    ch: {
      "meta.title": "clojure-native — 原生编译 Clojure，无需 JVM。",
      "meta.description":
        "clojure-native 将有明确文档的 Clojure 子集编译为独立的原生可执行文件，运行时无需 JVM。",
      "a11y.skip": "跳转到主要内容",
      "a11y.mainNav": "主导航",
      "a11y.footerNav": "页脚链接",
      "a11y.menuOpen": "打开菜单",
      "a11y.menuClose": "关闭菜单",
      "a11y.home": "clojure-native — 首页",
      "a11y.heroVisual": "将 Clojure 编译为原生可执行文件的示例",
      "a11y.proof": "核心技术与特性",
      "a11y.requirements": "环境要求",
      "language.label": "语言",
      "nav.project": "项目",
      "nav.pipeline": "编译流程",
      "nav.features": "特性",
      "nav.benchmarks": "基准测试",
      "nav.start": "快速开始",
      "nav.github": "GitHub",
      "hero.eyebrow": "实验性原生编译",
      "hero.titleHtml": "Clojure，<span class=\"accent\">原生编译。</span>无需 JVM。",
      "hero.descriptionHtml":
        "<strong>clojure-native</strong> 使用 Rust、Cranelift 和紧凑的 C runtime，将有明确文档的 Clojure 子集转换为独立的原生可执行文件。",
      "hero.primaryCta": "编译第一个程序",
      "hero.source": "查看源代码",
      "hero.codeComment": ";; Clojure 代码",
      "hero.status": "可执行文件已就绪 · JVM 0%",
      "proof.noJvm": "运行时无需 JVM",
      "proof.cranelift": "Cranelift 代码生成",
      "proof.gc": "精确 GC",
      "proof.open": "开放源代码",
      "project.kicker": "为什么要做",
      "project.quoteHtml": "探索一条<span>彻底原生</span>的路径，同时保留 Clojure 的表达力。",
      "project.binaryTitle": "独立二进制文件",
      "project.binaryBody":
        "Cranelift 生成的目标文件会与内置 C runtime 链接，最终程序的运行不依赖 JVM。",
      "project.scopeTitle": "明确的范围",
      "project.scopeBody":
        "兼容性、边界和架构决策都是产品文档的一部分，同时不会承诺完整实现。",
      "project.runtimeTitle": "轻量 runtime",
      "project.runtimeBody":
        "紧凑、单线程、非移动式的 C runtime，支持标签值、持久化集合和精确 mark-sweep GC。",
      "project.labTitle": "开放实验室",
      "project.labBody":
        "实现、符合性测试、基准测试和 ADR 都在同一个仓库中，让每一步演进都可验证。",
      "pipeline.kicker": "从文本到机器码",
      "pipeline.title": "精简的流程，清晰的每一步。",
      "pipeline.lead":
        "CLI 汇集读取、解释和 AOT 编译。在原生路径中，代码会经过四个定义清晰的层次。",
      "pipeline.input": "输入",
      "pipeline.readerBody": "将源代码转换为带 spans 的 forms，并支持 reader 宏、Unicode 和确定性诊断。",
      "pipeline.semantics": "语义",
      "pipeline.analyzerBody":
        "展开受支持的 core 宏，并解析函数、closures、records、协议和控制流。",
      "pipeline.code": "代码",
      "pipeline.craneliftBody":
        "生成机器码，为 fixnums 提供原生 fast paths，并直接访问 root shadow stack。",
      "pipeline.output": "输出",
      "pipeline.linkerTitle": "原生链接器",
      "pipeline.linkerBody": "将生成的目标文件与 C runtime 组合，为宿主平台生成可执行文件。",
      "features.kicker": "当前可用",
      "features.title": "不只是一个“hello world”实验。",
      "features.lead": "不断扩展的语言子集，并由可执行测试和兼容性文档提供支撑。",
      "features.memoryTag": "内存管理",
      "features.memoryTitle": "带 root shadow stack 的精确 GC",
      "features.memoryBody":
        "单线程、非移动式 mark-sweep，root 的加载和存储会直接生成到机器码中。",
      "features.collectionsTag": "不可变数据",
      "features.collectionsTitle": "真正的持久化集合",
      "features.collectionsBody":
        "基于 trie 的向量、带 HAMT 的混合 map 和 set、红黑树有序结构以及 transient 操作。",
      "features.languageTag": "语言",
      "features.functionsTitle": "函数与 closures",
      "features.functionsBody":
        "支持高阶函数、固定/多重/可变参数、递归、<code>apply</code> 和 <code>loop/recur</code>。",
      "features.abstractionsTag": "抽象",
      "features.protocolsTitle": "Protocols 与 records",
      "features.protocolsBody":
        "支持 <code>defrecord</code>、<code>defprotocol</code>、<code>extend-type</code> 以及内置类型 dispatch。",
      "features.coreTitle": "已编译的 core",
      "features.coreBody":
        "包含 26 个函数，例如 <code>map</code>、<code>filter</code>、<code>reduce</code>、<code>range</code>、<code>into</code> 和 <code>comp</code>。",
      "benchmarks.kicker": "可验证的性能",
      "benchmarks.title": "原生与 JVM，逐项对比。",
      "benchmarks.lead":
        "在相同负载和一致校验和下比较全部 90 个程序。无需离开页面，即可选择套件并切换时间、CPU 和内存指标。",
      "benchmarks.suiteLabel": "套件",
      "benchmarks.metricLabel": "指标",
      "benchmarks.suiteTabs": "选择基准测试套件",
      "benchmarks.metricTabs": "选择基准测试指标",
      "benchmarks.cormen": "Cormen / CLRS",
      "benchmarks.cracking": "Cracking",
      "benchmarks.wall": "时间",
      "benchmarks.cpu": "CPU",
      "benchmarks.memory": "内存",
      "benchmarks.summaryWall": "总时间 · 原生 / JVM",
      "benchmarks.summaryCpu": "总 CPU · 原生 / JVM",
      "benchmarks.summaryMemory": "RSS 中位数 · 原生 / JVM",
      "benchmarks.altWall": "{suite} 套件中原生可执行文件与 Clojure/JVM 的时间对比",
      "benchmarks.altCpu": "{suite} 套件中原生可执行文件与 Clojure/JVM 的 CPU 对比",
      "benchmarks.altMemory": "{suite} 套件中原生可执行文件与 Clojure/JVM 的内存对比",
      "benchmarks.captionWall":
        "绿色表示原生更优，橙色表示 JVM 更优。中线表示性能持平，采用对数刻度。",
      "benchmarks.captionCpu":
        "总 CPU 使用相同的持平线：右侧表示原生更优，左侧表示 JVM 更优。",
      "benchmarks.captionMemory":
        "蓝色表示原生，紫色表示 Clojure/JVM。数据点以对数刻度显示绝对 RSS（MiB）。",
      "benchmarks.openReport": "打开完整报告",
      "benchmarks.openCsv": "查看 CSV",
      "honesty.label": "有意保持实验性",
      "honesty.bodyHtml":
        "<strong>本项目目前还不能替代 Clojure/JVM。</strong> Java interop、bignums、惰性序列、动态 namespaces 和多文件项目仍不在当前原生路径的范围内。",
      "honesty.status": "v0.0.1 · 持续演进",
      "start.kicker": "几分钟即可开始",
      "start.title": "从克隆到原生二进制文件。",
      "start.lead": "以 release 模式构建 CLI，将任何兼容示例转换为适用于宿主平台的独立可执行文件。",
      "start.cCompiler": "C 编译器",
      "code.terminal": "终端",
      "code.copy": "复制",
      "code.copied": "已复制",
      "code.select": "请选择代码",
      "code.step1": "# 1. 获取项目",
      "code.step2": "# 2. 构建 CLI",
      "code.step3": "# 3. 生成并运行二进制文件",
      "stats.kicker": "可衡量的进展",
      "stats.title": "兼容性来自测试，而不是假设。",
      "stats.lead": "符合性矩阵记录已交付特性、已知缺口和待盘点项目，所有内容都可离线验证。",
      "stats.cases": "A–E 矩阵中的用例",
      "stats.active": "已启用",
      "stats.behaviors": "已实现行为",
      "stats.benchmarks": "算法基准测试",
      "stats.jvm": "二进制 runtime 中",
      "stats.note": "数据记录于仓库 · 版本 0.0.1",
      "cta.title": "探索。编译。贡献。",
      "cta.body": "代码、规范、基准测试和架构决策全部开放。查看当前状态，一起构建下一步。",
      "cta.button": "在 GitHub 上打开",
      "footer.note": "实验性项目，采用 MIT 或 Apache-2.0 许可证 ·",
      "footer.docs": "文档",
      "footer.specs": "规范",
      "footer.benchmarks": "基准测试",
      "footer.issues": "Issues",
    },
  };

  const supportedLanguages = Object.keys(languageConfig);
  const normalizeLanguage = (value) => {
    if (!value) return null;
    const normalized = value.toLowerCase().replace("_", "-");
    if (normalized === "zh" || normalized.startsWith("zh-") || normalized === "cn") {
      return "ch";
    }
    const shortCode = normalized.split("-")[0];
    return supportedLanguages.includes(shortCode) ? shortCode : null;
  };

  const readStoredLanguage = () => {
    try {
      return normalizeLanguage(window.localStorage.getItem("clojure-native-language"));
    } catch {
      return null;
    }
  };

  const detectBrowserLanguage = () => {
    const candidates = navigator.languages?.length ? navigator.languages : [navigator.language];
    return candidates.map(normalizeLanguage).find(Boolean) || null;
  };

  const url = new URL(window.location.href);
  let activeLanguage =
    normalizeLanguage(url.searchParams.get("lang")) ||
    readStoredLanguage() ||
    detectBrowserLanguage() ||
    "pt";

  const translate = (key) =>
    messages[activeLanguage]?.[key] ?? messages.pt[key] ?? key;

  const applyLanguage = (language, updateUrl = false) => {
    activeLanguage = normalizeLanguage(language) || "pt";
    const config = languageConfig[activeLanguage];

    document.documentElement.lang = config.htmlLang;
    document.documentElement.dir = "ltr";

    document.querySelectorAll("[data-i18n]").forEach((element) => {
      element.textContent = translate(element.dataset.i18n);
    });

    document.querySelectorAll("[data-i18n-html]").forEach((element) => {
      element.innerHTML = translate(element.dataset.i18nHtml);
    });

    document.querySelectorAll("[data-i18n-aria-label]").forEach((element) => {
      element.setAttribute("aria-label", translate(element.dataset.i18nAriaLabel));
    });

    document.querySelectorAll("[data-i18n-content]").forEach((element) => {
      element.setAttribute("content", translate(element.dataset.i18nContent));
    });

    const localeMeta = document.querySelector('meta[property="og:locale"]');
    if (localeMeta) localeMeta.setAttribute("content", config.locale);

    const languageButton = document.querySelector("#language-button");
    const currentFlag = document.querySelector("[data-current-language-flag]");
    const currentCode = document.querySelector("[data-current-language-code]");

    if (currentFlag) currentFlag.textContent = config.flag;
    if (currentCode) currentCode.textContent = config.code;
    if (languageButton) {
      languageButton.setAttribute("aria-label", `${translate("language.label")}: ${config.name}`);
    }

    document.querySelectorAll("[data-language]").forEach((option) => {
      option.setAttribute("aria-selected", String(option.dataset.language === activeLanguage));
    });

    try {
      window.localStorage.setItem("clojure-native-language", activeLanguage);
    } catch {
      // The page remains functional when storage is blocked.
    }

    if (updateUrl) {
      const nextUrl = new URL(window.location.href);
      nextUrl.searchParams.set("lang", activeLanguage);
      window.history.replaceState({}, "", nextUrl);
    }

    document.documentElement.classList.add("i18n-ready");
    window.dispatchEvent(
      new CustomEvent("page:languagechange", {
        detail: { language: activeLanguage, htmlLang: config.htmlLang },
      })
    );
  };

  const languageSwitcher = document.querySelector(".language-switcher");
  const languageButton = document.querySelector("#language-button");
  const languageMenu = document.querySelector("#language-menu");
  const languageOptions = [...document.querySelectorAll("[data-language]")];

  const setLanguageMenuOpen = (open) => {
    if (!languageButton || !languageMenu) return;
    languageButton.setAttribute("aria-expanded", String(open));
    languageMenu.hidden = !open;
  };

  const focusLanguageOption = (index) => {
    const normalizedIndex = (index + languageOptions.length) % languageOptions.length;
    languageOptions[normalizedIndex]?.focus();
  };

  languageButton?.addEventListener("click", () => {
    const willOpen = languageButton.getAttribute("aria-expanded") !== "true";
    setLanguageMenuOpen(willOpen);
    if (willOpen) {
      const activeIndex = languageOptions.findIndex(
        (option) => option.dataset.language === activeLanguage
      );
      window.requestAnimationFrame(() => focusLanguageOption(Math.max(activeIndex, 0)));
    }
  });

  languageOptions.forEach((option, index) => {
    option.addEventListener("click", () => {
      applyLanguage(option.dataset.language, true);
      setLanguageMenuOpen(false);
      languageButton?.focus();
    });

    option.addEventListener("keydown", (event) => {
      if (event.key === "ArrowDown" || event.key === "ArrowUp") {
        event.preventDefault();
        focusLanguageOption(index + (event.key === "ArrowDown" ? 1 : -1));
      }

      if (event.key === "Home" || event.key === "End") {
        event.preventDefault();
        focusLanguageOption(event.key === "Home" ? 0 : languageOptions.length - 1);
      }
    });
  });

  document.addEventListener("click", (event) => {
    if (!languageSwitcher?.contains(event.target)) setLanguageMenuOpen(false);
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && languageButton?.getAttribute("aria-expanded") === "true") {
      setLanguageMenuOpen(false);
      languageButton.focus();
    }
  });

  window.pageI18n = {
    apply: applyLanguage,
    get language() {
      return activeLanguage;
    },
    t: translate,
  };

  applyLanguage(activeLanguage);
})();
