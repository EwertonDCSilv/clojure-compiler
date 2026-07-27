SHELL := /usr/bin/env bash
.DEFAULT_GOAL := help
.NOTPARALLEL:

CARGO ?= cargo
INSTALL ?= install

CLI_PACKAGE := clojure-native-cli
CLI_NAME := clojure-native
CLI_BINARY := target/release/$(CLI_NAME)

PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin
DESTDIR ?=

CRACKING_ARGS ?=
CORMEN_ARGS ?=
CRACKING_COMPARE_ARGS ?=
CORMEN_COMPARE_ARGS ?=
CONFORMANCE_ARGS ?=
COVERAGE_ARGS ?=
BENCHMARK_OUTPUT_DIR ?= target/benchmarks
CRACKING_COMPARISON_CSV ?= $(BENCHMARK_OUTPUT_DIR)/cracking-comparison.csv
CORMEN_COMPARISON_CSV ?= $(BENCHMARK_OUTPUT_DIR)/cormen-comparison.csv

.PHONY: \
	help all ci quality \
	build release check \
	fmt fmt-check lint lint-rust lint-clojure \
	test test-runtime test-runtime-sanitize coverage \
	compatibility compatibility-list compatibility-oracle \
	benchmarks benchmarks-cracking benchmarks-cormen benchmarks-list \
	benchmarks-ci benchmarks-compare benchmarks-compare-cracking \
	benchmarks-compare-cormen \
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
		"  quality                  Formatação, lint Rust/Clojure e testes" \
		"  coverage                 Executa os gates de cobertura" \
		"  compatibility            Verifica a matriz de compatibilidade A–E" \
		"  benchmarks               Executa as suítes Cracking e Cormen" \
		"  benchmarks-compare       Compara as duas suítes com Clojure/JVM AOT" \
		"  install                  Instala clojure-native em ~/.local/bin" \
		"  all                      Executa qualidade, cobertura, compatibilidade e benchmarks" \
		"  ci                       Reproduz os comandos usados pelo GitHub Actions" \
		"" \
		"Alvos auxiliares:" \
		"  fmt | fmt-check          Formata ou valida o código Rust" \
		"  lint                     Executa os lints Rust e Clojure" \
		"  test-runtime             Testa o runtime C" \
		"  test-runtime-sanitize    Testa o runtime C com sanitizers" \
		"  compatibility-list       Lista casos da matriz de compatibilidade" \
		"  compatibility-oracle     Compara com o oracle Clojure/JVM manual" \
		"  benchmarks-list          Lista todos os casos das duas suítes" \
		"  benchmarks-ci            Executa o recorte de benchmarks usado no CI" \
		"" \
		"Variáveis úteis:" \
		"  CONFORMANCE_ARGS='--level A --status active'" \
		"  CRACKING_ARGS='--chapter 08 --scale 10'" \
		"  CORMEN_ARGS='--chapter 05'" \
		"  CRACKING_COMPARE_ARGS='--chapter 01 --scale 25'" \
		"  CORMEN_COMPARE_ARGS='--chapter 06 --scale 25'" \
		"  COVERAGE_ARGS='--html'" \
		"  CRACKING_COMPARISON_CSV=/tmp/cracking.csv" \
		"  CORMEN_COMPARISON_CSV=/tmp/cormen.csv" \
		"  PREFIX=/usr/local ou BINDIR=/outro/diretorio/bin" \
		"" \
		"Observação: coverage, all e ci exigem cargo-llvm-cov e llvm-tools-preview." \
		"" \
		"Aliases em português: compilar, testes, compatibilidade, instalar"

all: quality coverage compatibility benchmarks

ci: quality coverage benchmarks-ci compatibility

quality: fmt-check lint test

build:
	$(CARGO) build --workspace

release:
	$(CARGO) build --release -p $(CLI_PACKAGE)

check:
	$(CARGO) check --workspace --all-targets

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all --check

lint: lint-rust lint-clojure

lint-rust:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

lint-clojure:
	scripts/lint-clojure.sh

test:
	$(CARGO) test --workspace

test-runtime:
	scripts/test-runtime-c.sh

test-runtime-sanitize:
	scripts/test-runtime-c.sh --sanitize

coverage:
	scripts/coverage.sh $(COVERAGE_ARGS)

compatibility:
	scripts/conformance.sh verify $(CONFORMANCE_ARGS)

compatibility-list:
	scripts/conformance.sh list $(CONFORMANCE_ARGS)

compatibility-oracle:
	scripts/conformance.sh oracle --check $(CONFORMANCE_ARGS)

benchmarks: benchmarks-cracking benchmarks-cormen

benchmarks-cracking: release
	benchmarks/cracking/run.sh $(CRACKING_ARGS)

benchmarks-cormen: release
	benchmarks/cormen/run.sh $(CORMEN_ARGS)

benchmarks-list:
	benchmarks/cracking/run.sh --list $(CRACKING_ARGS)
	benchmarks/cormen/run.sh --list $(CORMEN_ARGS)

benchmarks-ci: release
	benchmarks/cracking/run.sh --chapter 01
	benchmarks/cormen/run.sh

benchmarks-compare: benchmarks-compare-cracking benchmarks-compare-cormen

benchmarks-compare-cracking: release
	mkdir -p "$(dir $(CRACKING_COMPARISON_CSV))"
	benchmarks/cracking/compare-clojure.sh --csv "$(CRACKING_COMPARISON_CSV)" $(CRACKING_COMPARE_ARGS)

benchmarks-compare-cormen: release
	mkdir -p "$(dir $(CORMEN_COMPARISON_CSV))"
	benchmarks/cormen/compare-clojure.sh --csv "$(CORMEN_COMPARISON_CSV)" $(CORMEN_COMPARE_ARGS)

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
