SHELL := /usr/bin/env bash
.DEFAULT_GOAL := help
.NOTPARALLEL:

CARGO ?= cargo
RUSTC ?= rustc
INSTALL ?= install

CLI_PACKAGE := clojure-native-cli
CLI_NAME := clojure-native
CLI_BINARY := target/release/$(CLI_NAME)
BENCHMARK_CHART_RENDERER := target/benchmark-tools/render-benchmark-charts
BENCHMARK_CHART_TESTS := target/benchmark-tools/render-benchmark-charts-tests
IR_AB_ANALYZER_TESTS := target/benchmark-tools/analyze-ir-ab-tests

PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin
DESTDIR ?=

CRACKING_ARGS ?=
CORMEN_ARGS ?=
CORMEN_IR_ARGS ?=
EXERCISM_ARGS ?=
CRACKING_COMPARE_ARGS ?=
CORMEN_COMPARE_ARGS ?=
EXERCISM_COMPARE_ARGS ?=
CONFORMANCE_ARGS ?=
COVERAGE_ARGS ?=
AGENT_GUARD_ARGS ?=
BENCHMARK_OUTPUT_DIR ?= target/benchmarks
CRACKING_COMPARISON_CSV ?= $(BENCHMARK_OUTPUT_DIR)/cracking-comparison.csv
CORMEN_COMPARISON_CSV ?= $(BENCHMARK_OUTPUT_DIR)/cormen-comparison.csv
EXERCISM_COMPARISON_CSV ?= $(BENCHMARK_OUTPUT_DIR)/exercism-comparison.csv

.PHONY: \
	help all ci quality docs-check pre-commit pre-push hooks-install agent-feature-guard \
	build release check \
	fmt fmt-check lint lint-files lint-rust lint-c lint-clojure \
	test test-runtime test-runtime-sanitize test-agent-guard test-benchmark-charts test-ir-ab-analyzer coverage \
	compatibility reader-syntax-coverage compatibility-list compatibility-oracle \
	exercism-compatibility \
	benchmarks benchmarks-cracking benchmarks-cormen benchmarks-cormen-ir benchmarks-exercism benchmarks-list \
	benchmarks-charts \
	benchmarks-ci benchmarks-compare benchmarks-compare-cracking \
	benchmarks-compare-cormen benchmarks-compare-exercism \
	install \
	compilar testes compatibilidade instalar

help:
	@printf '%s\n' \
		"clojure-compiler — rotinas do projeto" \
		"" \
		"Uso: make <alvo> [VAR=valor]" \
		"" \
		"Principais alvos:" \
		"  build                    Compila todo o workspace em modo debug" \
		"  release                  Compila o CLI otimizado em target/release" \
		"  test                     Executa cargo test no workspace" \
		"  quality                  Formatação, documentação, lint e testes" \
		"  docs-check               Valida rustdoc, doctests e contratos documentais" \
		"  pre-commit               Executa higiene, formato e lints locais" \
		"  pre-push                 Executa quality e sanitizers do runtime C" \
		"  hooks-install            Ativa os hooks Git versionados neste checkout" \
		"  agent-feature-guard      Bloqueia features sem estimativa ou acima de 8 pontos" \
		"  coverage                 Executa os gates de cobertura" \
		"  compatibility            Verifica a matriz de compatibilidade A–E" \
		"  reader-syntax-coverage   Mede suporte do Reader Clojure 1.12.5" \
		"  benchmarks               Executa as suítes Cracking, Cormen e Exercism" \
		"  benchmarks-cormen-ir     Executa o gate A/B da IR opcional no Cormen" \
		"  benchmarks-charts        Atualiza os gráficos SVG dos resultados" \
		"  benchmarks-compare       Compara as três suítes com Clojure/JVM AOT" \
		"  exercism-compatibility   Audita 101 práticas, 13 conceitos e 493 arquivos Exercism" \
		"  install                  Instala clojure-native em ~/.local/bin" \
		"  all                      Executa qualidade, cobertura, compatibilidade e benchmarks" \
		"  ci                       Reproduz os comandos usados pelo GitHub Actions" \
		"" \
		"Alvos auxiliares:" \
		"  fmt | fmt-check          Formata ou valida o código Rust" \
		"  lint                     Executa higiene e lints Rust, C e Clojure" \
		"  test-runtime             Testa o runtime C" \
		"  test-runtime-sanitize    Testa o runtime C com sanitizers" \
		"  test-agent-guard         Testa o guard rail de story points sem rede" \
		"  test-benchmark-charts    Testa o gerador Rust de gráficos" \
		"  test-ir-ab-analyzer      Testa o analisador estatístico do gate de IR" \
		"  compatibility-list       Lista casos da matriz de compatibilidade" \
		"  compatibility-oracle     Compara com o oracle Clojure/JVM manual" \
		"  benchmarks-list          Lista todos os casos das três suítes" \
		"  benchmarks-ci            Executa o recorte de benchmarks usado no CI" \
		"" \
		"Variáveis úteis:" \
		"  CONFORMANCE_ARGS='--level A --status active'" \
		"  CRACKING_ARGS='--chapter 08 --scale 10'" \
		"  CORMEN_ARGS='--chapter 05'" \
		"  CORMEN_IR_ARGS='--repetitions 7 --scale 25'" \
		"  EXERCISM_ARGS='--scale 5'" \
		"  CRACKING_COMPARE_ARGS='--chapter 01 --scale 25'" \
		"  CORMEN_COMPARE_ARGS='--chapter 06 --scale 25'" \
		"  EXERCISM_COMPARE_ARGS='--scale 5'" \
		"  COVERAGE_ARGS='--html'" \
		"  ISSUE=101 make agent-feature-guard" \
		"  CRACKING_COMPARISON_CSV=/tmp/cracking.csv" \
		"  CORMEN_COMPARISON_CSV=/tmp/cormen.csv" \
		"  EXERCISM_COMPARISON_CSV=/tmp/exercism.csv" \
		"  PREFIX=/usr/local ou BINDIR=/outro/diretorio/bin" \
		"" \
		"Observação: coverage, all e ci exigem cargo-llvm-cov e llvm-tools-preview." \
		"" \
		"Aliases em português: compilar, testes, compatibilidade, instalar"

all: quality coverage compatibility benchmarks

ci: quality coverage benchmarks-ci compatibility

quality: fmt-check docs-check lint test

docs-check:
	scripts/check-docs.sh

pre-commit:
	scripts/pre-commit.sh --all

pre-push: quality test-runtime-sanitize

hooks-install:
	scripts/install-git-hooks.sh

agent-feature-guard:
	scripts/check-agent-story-points.sh $(if $(ISSUE),--issue $(ISSUE)) $(AGENT_GUARD_ARGS)

build:
	$(CARGO) build --workspace

release:
	$(CARGO) build --release -p $(CLI_PACKAGE)

check:
	$(CARGO) check --workspace --all-targets

fmt:
	$(CARGO) fmt --all
	rustfmt --edition 2021 benchmarks/render-benchmark-charts.rs
	rustfmt --edition 2021 benchmarks/analyze-ir-ab.rs

fmt-check:
	$(CARGO) fmt --all --check
	rustfmt --edition 2021 --check benchmarks/render-benchmark-charts.rs
	rustfmt --edition 2021 --check benchmarks/analyze-ir-ab.rs

lint: lint-files lint-rust lint-c lint-clojure

lint-files:
	scripts/check-file-hygiene.sh --tracked

lint-rust:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

lint-c:
	scripts/lint-c.sh

lint-clojure:
	scripts/lint-clojure.sh

test: test-agent-guard test-benchmark-charts test-ir-ab-analyzer
	$(CARGO) test --workspace

test-agent-guard:
	tests/scripts/check-agent-story-points.sh

test-runtime:
	scripts/test-runtime-c.sh

test-runtime-sanitize:
	scripts/test-runtime-c.sh --sanitize

fuzz-http:
	scripts/fuzz-http.sh $(FUZZ_SECONDS)

$(IR_AB_ANALYZER_TESTS): benchmarks/analyze-ir-ab.rs
	mkdir -p "$(dir $@)"
	$(RUSTC) --edition=2021 -D warnings --test "$<" -o "$@"

test-ir-ab-analyzer: $(IR_AB_ANALYZER_TESTS)
	$(IR_AB_ANALYZER_TESTS)

coverage:
	scripts/coverage.sh $(COVERAGE_ARGS)

compatibility:
	scripts/conformance.sh reader-coverage
	scripts/conformance.sh verify $(CONFORMANCE_ARGS)

reader-syntax-coverage:
	scripts/conformance.sh reader-coverage $(CONFORMANCE_ARGS)

compatibility-list:
	scripts/conformance.sh list $(CONFORMANCE_ARGS)

compatibility-oracle:
	scripts/conformance.sh oracle --check $(CONFORMANCE_ARGS)

exercism-compatibility: release
	benchmarks/exercism/compile-all.sh
	benchmarks/exercism/compile-all.sh \
		--scope concepts \
		--report tests/conformance/level-d-pure-libraries/external/exercism/compilation.tsv
	benchmarks/exercism/compile-all.sh \
		--scope all \
		--report benchmarks/exercism/results/all-files.tsv

benchmarks: benchmarks-cracking benchmarks-cormen benchmarks-exercism

benchmarks-cracking: release
	benchmarks/cracking/run.sh $(CRACKING_ARGS)

benchmarks-cormen: release
	benchmarks/cormen/run.sh $(CORMEN_ARGS)

benchmarks-cormen-ir: release
	benchmarks/cormen/compare-ir.sh $(CORMEN_IR_ARGS)

benchmarks-exercism: release
	benchmarks/exercism/run.sh $(EXERCISM_ARGS)

benchmarks-list:
	benchmarks/cracking/run.sh --list $(CRACKING_ARGS)
	benchmarks/cormen/run.sh --list $(CORMEN_ARGS)
	benchmarks/exercism/run.sh --list $(EXERCISM_ARGS)

$(BENCHMARK_CHART_RENDERER): benchmarks/render-benchmark-charts.rs
	mkdir -p "$(dir $@)"
	$(RUSTC) --edition=2021 -D warnings -O "$<" -o "$@"

$(BENCHMARK_CHART_TESTS): benchmarks/render-benchmark-charts.rs
	mkdir -p "$(dir $@)"
	$(RUSTC) --edition=2021 -D warnings --test "$<" -o "$@"

test-benchmark-charts: $(BENCHMARK_CHART_TESTS)
	$(BENCHMARK_CHART_TESTS)

benchmarks-charts: $(BENCHMARK_CHART_RENDERER)
	$(BENCHMARK_CHART_RENDERER) \
		benchmarks/cracking/results/extreme.csv \
		benchmarks/cracking/results/charts \
		"Cracking"
	$(BENCHMARK_CHART_RENDERER) \
		benchmarks/cormen/results/extreme.csv \
		benchmarks/cormen/results/charts \
		"Cormen/CLRS"
	$(BENCHMARK_CHART_RENDERER) \
		benchmarks/exercism/results/extreme.csv \
		benchmarks/exercism/results/charts \
		"Exercism"
	$(BENCHMARK_CHART_RENDERER) \
		benchmarks/cracking/results/extreme.csv \
		docs/assets/benchmarks/cracking \
		"Cracking"
	$(BENCHMARK_CHART_RENDERER) \
		benchmarks/cormen/results/extreme.csv \
		docs/assets/benchmarks/cormen \
		"Cormen/CLRS"
	$(BENCHMARK_CHART_RENDERER) \
		benchmarks/exercism/results/extreme.csv \
		docs/assets/benchmarks/exercism \
		"Exercism"

benchmarks-ci: release
	benchmarks/cracking/run.sh --chapter 01
	benchmarks/cormen/run.sh

benchmarks-compare: benchmarks-compare-cracking benchmarks-compare-cormen benchmarks-compare-exercism

benchmarks-compare-cracking: release
	mkdir -p "$(dir $(CRACKING_COMPARISON_CSV))"
	benchmarks/cracking/compare-clojure.sh --csv "$(CRACKING_COMPARISON_CSV)" $(CRACKING_COMPARE_ARGS)

benchmarks-compare-cormen: release
	mkdir -p "$(dir $(CORMEN_COMPARISON_CSV))"
	benchmarks/cormen/compare-clojure.sh --csv "$(CORMEN_COMPARISON_CSV)" $(CORMEN_COMPARE_ARGS)

benchmarks-compare-exercism: release
	mkdir -p "$(dir $(EXERCISM_COMPARISON_CSV))"
	benchmarks/exercism/compare-clojure.sh --csv "$(EXERCISM_COMPARISON_CSV)" $(EXERCISM_COMPARE_ARGS)

install: release
	@test -n "$(BINDIR)" || { printf '%s\n' "BINDIR não pode ser vazio." >&2; exit 2; }
	$(INSTALL) -d "$(DESTDIR)$(BINDIR)"
	$(INSTALL) -m 0755 "$(CLI_BINARY)" "$(DESTDIR)$(BINDIR)/$(CLI_NAME)"
	@printf 'Instalado: %s\n' "$(DESTDIR)$(BINDIR)/$(CLI_NAME)"
	@case ":$$PATH:" in \
		*":$(BINDIR):"*) ;; \
		*) printf 'Aviso: adicione %s ao PATH para chamar %s diretamente.\n' \
			"$(BINDIR)" "$(CLI_NAME)" ;; \
	esac

compilar: release

testes: test

compatibilidade: compatibility

instalar: install
