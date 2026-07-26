//! Regenerates the tracked conformance fixture matrix.
//!
//! This is intentionally an explicit maintainer command. Normal verification
//! never rewrites fixtures.

use clojure_test_support::update_checksums;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
enum Expected {
    Edn(&'static str),
    Stdout(&'static str),
    Stderr(&'static str),
    None,
}

#[derive(Clone, Copy)]
struct Fixture {
    level: char,
    area_directory: &'static str,
    slug: &'static str,
    id: &'static str,
    area: &'static str,
    status: &'static str,
    class: &'static str,
    target: &'static str,
    oracle: &'static str,
    gc_stress: bool,
    reason: &'static str,
    tracking: &'static str,
    namespace: Option<&'static str>,
    input: &'static str,
    expected: Expected,
}

fn main() {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = repository.join("tests/conformance");
    for fixture in fixtures() {
        write_fixture(&root, &fixture);
    }
    let digest = update_checksums(&root).expect("update suite checksums");
    println!("generated conformance suite at {}", root.display());
    println!("checksum manifest: {digest}");
}

fn write_fixture(root: &Path, fixture: &Fixture) {
    let level_directory = match fixture.level {
        'A' => "level-a-syntax",
        'B' => "level-b-semantics",
        'C' => "level-c-stdlib",
        'D' => "level-d-pure-libraries",
        'E' => "level-e-ecosystem",
        other => panic!("unknown level {other}"),
    };
    let mut directory = root.join(level_directory);
    if !fixture.area_directory.is_empty() {
        directory = directory.join(fixture.area_directory);
    }
    directory = directory.join(fixture.slug);
    fs::create_dir_all(&directory).expect("create fixture directory");
    let namespace = fixture
        .namespace
        .map(|value| format!("namespace = \"{value}\"\n"))
        .unwrap_or_default();
    let manifest = format!(
        "id = \"{}\"\n\
         level = \"{}\"\n\
         area = \"{}\"\n\
         status = \"{}\"\n\
         class = \"{}\"\n\
         target = \"{}\"\n\
         oracle = \"{}\"\n\
         timeout_ms = 10000\n\
         gc_stress = {}\n\
         reason = \"{}\"\n\
         tracking = \"{}\"\n\
         {}",
        fixture.id,
        fixture.level,
        fixture.area,
        fixture.status,
        fixture.class,
        fixture.target,
        fixture.oracle,
        fixture.gc_stress,
        fixture.reason,
        fixture.tracking,
        namespace
    );
    fs::write(directory.join("case.toml"), manifest).expect("write case.toml");
    fs::write(directory.join("input.clj"), fixture.input).expect("write input.clj");
    match fixture.expected {
        Expected::Edn(value) => {
            fs::write(directory.join("expected.edn"), value).expect("write expected.edn")
        }
        Expected::Stdout(value) => {
            fs::write(directory.join("expected.stdout"), value).expect("write expected.stdout")
        }
        Expected::Stderr(value) => {
            fs::write(directory.join("expected.stderr"), value).expect("write expected.stderr")
        }
        Expected::None => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn fixture(
    level: char,
    area_directory: &'static str,
    slug: &'static str,
    id: &'static str,
    area: &'static str,
    status: &'static str,
    class: &'static str,
    target: &'static str,
    oracle: &'static str,
    gc_stress: bool,
    reason: &'static str,
    tracking: &'static str,
    namespace: Option<&'static str>,
    input: &'static str,
    expected: Expected,
) -> Fixture {
    Fixture {
        level,
        area_directory,
        slug,
        id,
        area,
        status,
        class,
        target,
        oracle,
        gc_stress,
        reason,
        tracking,
        namespace,
        input,
        expected,
    }
}

fn reader(
    area: &'static str,
    slug: &'static str,
    input: &'static str,
    expected: &'static str,
) -> Fixture {
    fixture(
        'A',
        area,
        slug,
        leak(format!(
            "a.{}.{}",
            area.replace('-', "_"),
            slug.replace('-', "_")
        )),
        leak(format!("syntax/{area}")),
        "active",
        "spec",
        "reader",
        "equal",
        false,
        "Implemented by the current reader.",
        "specs/LANGUAGE_SCOPE.md#reader",
        None,
        input,
        Expected::Edn(expected),
    )
}

fn reader_xfail(
    area: &'static str,
    slug: &'static str,
    input: &'static str,
    desired: &'static str,
    tracking: &'static str,
) -> Fixture {
    fixture(
        'A',
        area,
        slug,
        leak(format!(
            "a.{}.{}",
            area.replace('-', "_"),
            slug.replace('-', "_")
        )),
        leak(format!("syntax/{area}")),
        "xfail",
        "unsupported",
        "reader",
        "not-applicable",
        false,
        "The syntax is known but the current reader rejects it.",
        tracking,
        None,
        input,
        Expected::Edn(desired),
    )
}

fn reader_expected_diff(
    area: &'static str,
    slug: &'static str,
    input: &'static str,
    expected: &'static str,
) -> Fixture {
    let mut value = reader(area, slug, input, expected);
    value.class = "expected-diff";
    value.oracle = "expected-diff";
    value.reason = "The native reader preserves metadata syntax directly; the JVM normalizes it to a metadata map.";
    value
}

fn reader_without_oracle(
    area: &'static str,
    slug: &'static str,
    input: &'static str,
    expected: &'static str,
    reason: &'static str,
) -> Fixture {
    let mut value = reader(area, slug, input, expected);
    value.oracle = "not-applicable";
    value.reason = reason;
    value
}

fn diagnostic(slug: &'static str, input: &'static str, code: &'static str) -> Fixture {
    fixture(
        'A',
        "diagnostics",
        slug,
        leak(format!("a.diagnostics.{}", slug.replace('-', "_"))),
        "syntax/diagnostics",
        "active",
        "spec",
        "build-error",
        "not-applicable",
        false,
        "The reader must reject malformed input with a stable category.",
        "crates/clojure-reader/src/lib.rs",
        None,
        input,
        Expected::Stderr(code),
    )
}

fn build(
    area: &'static str,
    slug: &'static str,
    body: &'static str,
    expected: &'static str,
) -> Fixture {
    fixture(
        'B',
        area,
        slug,
        leak(format!(
            "b.{}.{}",
            area.replace('-', "_"),
            slug.replace('-', "_")
        )),
        leak(format!("semantics/{area}")),
        "active",
        "spec",
        "build-run",
        "equal",
        false,
        "Implemented by the current analyzer, code generator, and runtime.",
        "specs/COMPATIBILITY_SPEC.md#nível-b",
        None,
        body,
        Expected::Stdout(expected),
    )
}

fn build_gc(
    area: &'static str,
    slug: &'static str,
    body: &'static str,
    expected: &'static str,
) -> Fixture {
    let mut value = build(area, slug, body, expected);
    value.gc_stress = true;
    value
}

fn build_expected_diff(
    area: &'static str,
    slug: &'static str,
    body: &'static str,
    expected: &'static str,
    reason: &'static str,
) -> Fixture {
    let mut value = build(area, slug, body, expected);
    value.class = "expected-diff";
    value.oracle = "expected-diff";
    value.reason = reason;
    value
}

fn build_xfail(
    area: &'static str,
    slug: &'static str,
    body: &'static str,
    desired: &'static str,
    tracking: &'static str,
) -> Fixture {
    fixture(
        'B',
        area,
        slug,
        leak(format!(
            "b.{}.{}",
            area.replace('-', "_"),
            slug.replace('-', "_")
        )),
        leak(format!("semantics/{area}")),
        "xfail",
        "unsupported",
        "build-run",
        "equal",
        false,
        "The feature has a defined Clojure result but no current native execution path.",
        tracking,
        None,
        body,
        Expected::Stdout(desired),
    )
}

fn build_error(slug: &'static str, input: &'static str, code: &'static str) -> Fixture {
    fixture(
        'B',
        "errors",
        slug,
        leak(format!("b.errors.{}", slug.replace('-', "_"))),
        "semantics/errors",
        "active",
        "spec",
        "build-error",
        "not-applicable",
        false,
        "The analyzer must reject the invalid program with a stable category.",
        "crates/clojure-analyzer/src/lib.rs",
        None,
        input,
        Expected::Stderr(code),
    )
}

fn core(slug: &'static str, body: &'static str, expected: &'static str) -> Fixture {
    fixture(
        'C',
        "clojure-core",
        slug,
        leak(format!("c.clojure_core.{}", slug.replace('?', "_q").replace('-', "_"))),
        "stdlib/clojure-core",
        "active",
        "official",
        "build-run",
        "equal",
        false,
        "The function is compiled from the current embedded clojure.core subset; the case has normal, boundary, and alternate-input scenarios.",
        "crates/clojure-native-cli/src/core_compiled.clj",
        Some("clojure.core"),
        body,
        Expected::Stdout(expected),
    )
}

fn core_invalid_arity(slug: &'static str, function: &'static str) -> Fixture {
    fixture(
        'C',
        "clojure-core",
        leak(format!("{slug}-invalid-arity")),
        leak(format!(
            "c.clojure_core.{}.invalid_arity",
            slug.replace('?', "_q").replace('-', "_")
        )),
        "stdlib/clojure-core",
        "active",
        "official",
        "build-error",
        "not-applicable",
        false,
        "The active function group includes an invalid-arity scenario with a stable compiler diagnostic.",
        "crates/clojure-native-cli/src/core_compiled.clj",
        Some("clojure.core"),
        leak(format!(
            "(ns c.invalid-arity)\n(defn -main [] (println ({function})))\n(-main)\n"
        )),
        Expected::Stderr("E0103"),
    )
}

fn pending_stdlib(
    namespace_directory: &'static str,
    namespace: &'static str,
    function: &'static str,
    input: &'static str,
) -> Fixture {
    fixture(
        'C',
        namespace_directory,
        leak(function.replace('?', "_q").replace('!', "_bang")),
        leak(format!(
            "c.{}.{}",
            namespace.replace('.', "_"),
            function.replace('?', "_q").replace('!', "_bang")
        )),
        leak(format!("stdlib/{namespace_directory}")),
        "pending",
        "unsupported",
        "build-run",
        "equal",
        false,
        "The namespace is documented but is not loadable through the compiled path yet.",
        "specs/STANDARD_LIBRARY_SCOPE.md#namespaces-avaliados",
        Some(namespace),
        input,
        Expected::None,
    )
}

fn pending_project(
    level: char,
    slug: &'static str,
    area: &'static str,
    reason: &'static str,
    input: &'static str,
) -> Fixture {
    fixture(
        level,
        "",
        slug,
        leak(format!(
            "{}.{}",
            level.to_ascii_lowercase(),
            slug.replace('-', "_")
        )),
        area,
        "pending",
        "unsupported",
        "project",
        "not-applicable",
        false,
        reason,
        "specs/COMPATIBILITY_SPEC.md",
        None,
        input,
        Expected::None,
    )
}

fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn fixtures() -> Vec<Fixture> {
    let mut cases = vec![
        // Level A — literals.
        reader("literals", "integers", "0 -1 1 42\n", "0\n-1\n1\n42\n"),
        reader(
            "literals",
            "integer-limits",
            "-9223372036854775808 9223372036854775807\n",
            "-9223372036854775808\n9223372036854775807\n",
        ),
        reader("literals", "floats", "0.0 -1.5 6.02e23\n", "0.0\n-1.5\n602000000000000000000000.0\n"),
        reader(
            "literals",
            "strings-escapes",
            "\"plain\" \"line\\nfeed\" \"quote: \\\" slash: \\\\\"\n",
            "\"plain\"\n\"line\\nfeed\"\n\"quote: \\\" slash: \\\\\"\n",
        ),
        reader("literals", "strings-unicode", "\"ação λ 東京 😀\"\n", "\"ação λ 東京 😀\"\n"),
        reader("literals", "characters", "\\a \\newline \\space \\tab \\u03bb\n", "\\a\n\\newline\n\\space\n\\tab\n\\λ\n"),
        reader("literals", "symbols", "alpha foo/bar + <= *x*\n", "alpha\nfoo/bar\n+\n<=\n*x*\n"),
        reader("literals", "keywords", ":alpha :foo/bar :kebab-case\n", ":alpha\n:foo/bar\n:kebab-case\n"),
        reader("literals", "booleans-nil", "true false nil\n", "true\nfalse\nnil\n"),
        reader_xfail("literals", "ratio", "22/7\n", "22/7\n", "specs/LANGUAGE_SCOPE.md#números--política-explícita-decisão"),
        reader_xfail("literals", "bigint", "42N\n", "42N\n", "specs/LANGUAGE_SCOPE.md#números--política-explícita-decisão"),
        reader_xfail("literals", "auto-keyword", "::local\n", "::local\n", "specs/LANGUAGE_SCOPE.md#reader"),
        // Level A — collections.
        reader("collections", "lists", "() (1 2 three)\n", "()\n(1 2 three)\n"),
        reader("collections", "vectors", "[] [1 :two \"three\"]\n", "[]\n[1 :two \"three\"]\n"),
        reader("collections", "maps", "{} {:a 1 :b 2}\n", "{}\n{:a 1, :b 2}\n"),
        reader("collections", "sets", "#{} #{1 2 :three}\n", "#{}\n#{1 2 :three}\n"),
        reader(
            "collections",
            "nesting",
            "{:vector [1 (2 #{3})] :map {:nested true}}\n",
            "{:vector [1 (2 #{3})], :map {:nested true}}\n",
        ),
        reader("collections", "map-commas", "{:a,1,:b,2}\n", "{:a 1, :b 2}\n"),
        // Level A — reader macros and metadata.
        reader("reader-macros", "quote", "'(a b)\n", "(quote (a b))\n"),
        reader("reader-macros", "deref", "@state\n", "(deref state)\n"),
        reader("reader-macros", "var-quote", "#'alpha\n", "(var alpha)\n"),
        reader("reader-macros", "discard", "1 #_[:ignored true] 2\n", "1\n2\n"),
        reader_without_oracle(
            "reader-macros",
            "anonymous-function",
            "#(+ %1 %2)\n",
            "(fn* [%1 %2] (+ %1 %2))\n",
            "Implemented with deterministic argument names; JVM-generated names are intentionally not compared.",
        ),
        reader_xfail("reader-macros", "syntax-quote", "`(a ~b)\n", "(a b)\n", "specs/LANGUAGE_SCOPE.md#reader"),
        reader_xfail("reader-macros", "regex", "#\"a+\"\n", "#\"a+\"\n", "specs/LANGUAGE_SCOPE.md#reader"),
        reader_xfail("reader-macros", "reader-conditional", "#?(:cljn 1 :clj 2)\n", "1\n", "specs/LANGUAGE_SCOPE.md#reader"),
        reader_expected_diff("metadata", "keyword", "^:private value\n", "^:private value\n"),
        reader_expected_diff("metadata", "symbol", "^String value\n", "^String value\n"),
        reader("metadata", "map", "^{:doc \"x\"} value\n", "^{:doc \"x\"} value\n"),
        // Level A — trivia.
        reader("trivia", "comments", "; before\n1 ; after\n2\n", "1\n2\n"),
        reader_without_oracle(
            "trivia",
            "shebang",
            "#!/usr/bin/env clojure\n1\n",
            "1\n",
            "Shebang handling belongs to the source-file entry path, not the JVM EDN reader helper.",
        ),
        reader("trivia", "commas-whitespace", "1,\t2,\n3\n", "1\n2\n3\n"),
        reader("trivia", "nested-discard", "#_#_1 2 3\n", "3\n"),
        // Level A — diagnostics.
        diagnostic("unexpected-delimiter", ")\n", "E0003"),
        diagnostic("unclosed-vector", "[1 2\n", "E0004"),
        diagnostic("mismatched-delimiter", "[1}\n", "E0005"),
        diagnostic("odd-map", "{:a 1 :b}\n", "E0006"),
        diagnostic("unterminated-string", "\"open\n", "E0007"),
        diagnostic("bad-unicode-escape", "\"\\u12x4\"\n", "E0008"),
        diagnostic("bad-string-escape", "\"\\q\"\n", "E0009"),
        diagnostic("unquote-outside-syntax-quote", "~@x\n", "E0011"),
        diagnostic("unknown-dispatch", "#z\n", "E0014"),
        diagnostic("empty-character", "\\", "E0015"),
        diagnostic("unknown-character", "\\unknown\n", "E0016"),
        diagnostic("empty-keyword", ":\n", "E0018"),
        diagnostic("invalid-number", "12oops\n", "E0022"),
        // Level B — arithmetic.
        build("arithmetic", "addition", "(ns b.add)\n(defn -main [] (println (+ 1 2 3) (+ -5 5) (+ 0 0)))\n(-main)\n", "6 0 0\n"),
        build("arithmetic", "subtraction", "(ns b.sub)\n(defn -main [] (println (- 10 3 2) (- -2 -3) (- 0 0)))\n(-main)\n", "5 1 0\n"),
        build("arithmetic", "multiplication", "(ns b.mul)\n(defn -main [] (println (* 6 7) (* -3 4) (* 0 99)))\n(-main)\n", "42 -12 0\n"),
        build("arithmetic", "quot-mod", "(ns b.div)\n(defn -main [] (println (quot 17 5) (mod 17 5) (quot -17 5) (mod -17 5)))\n(-main)\n", "3 2 -3 3\n"),
        build("arithmetic", "inc-dec", "(ns b.step)\n(defn -main [] (println (inc 0) (dec 0) (inc -2) (dec 2)))\n(-main)\n", "1 -1 -1 1\n"),
        build("arithmetic", "comparisons", "(ns b.cmp)\n(defn -main [] (println (< 1 2) (<= 2 2) (> 3 2) (>= 2 3) (= 4 4)))\n(-main)\n", "true true true false true\n"),
        build("arithmetic", "numeric-predicates", "(ns b.pred)\n(defn -main [] (println (zero? 0) (pos? 2) (neg? -2) (even? 4) (odd? 5)))\n(-main)\n", "true true true true true\n"),
        build_xfail("arithmetic", "overflow", "(ns b.overflow)\n(defn -main [] (println (+ 4611686018427387903 1)))\n(-main)\n", "4611686018427387904\n", "specs/LANGUAGE_SCOPE.md#números--política-explícita-decisão"),
        build_xfail("arithmetic", "float-codegen", "(ns b.float)\n(defn -main [] (println (+ 1.5 2.5)))\n(-main)\n", "4.0\n", "specs/LANGUAGE_SCOPE.md#números--política-explícita-decisão"),
        // Level B — control flow.
        build("control-flow", "truthiness", "(ns b.truth)\n(defn -main [] (println (if nil :bad :nil-false) (if false :bad :false-false) (if 0 :zero-true :bad) (if \"\" :string-true :bad)))\n(-main)\n", ":nil-false :false-false :zero-true :string-true\n"),
        build("control-flow", "if", "(ns b.if)\n(defn choose [x] (if x 1 2))\n(defn -main [] (println (choose true) (choose false) (choose nil)))\n(-main)\n", "1 2 2\n"),
        build("control-flow", "do", "(ns b.do)\n(defn -main [] (println (do 1 2 3)) (do (print \"a\") (println \"b\")))\n(-main)\n", "3\nab\n"),
        build("control-flow", "let", "(ns b.let)\n(defn -main [] (println (let [a 20 b 22] (+ a b)) (let [x 1] (let [x 2] x))))\n(-main)\n", "42 2\n"),
        build("control-flow", "loop-recur", "(ns b.loop)\n(defn sum [n] (loop [i 0 acc 0] (if (> i n) acc (recur (inc i) (+ acc i)))))\n(defn -main [] (println (sum 0) (sum 10) (sum 100)))\n(-main)\n", "0 55 5050\n"),
        build("control-flow", "cond", "(ns b.cond)\n(defn sign [x] (cond (< x 0) :neg (= x 0) :zero :else :pos))\n(defn -main [] (println (sign -1) (sign 0) (sign 1)))\n(-main)\n", ":neg :zero :pos\n"),
        build("control-flow", "when", "(ns b.when)\n(defn -main [] (when true (print \"yes\")) (when false (print \"no\")) (println))\n(-main)\n", "yes\n"),
        build("control-flow", "and-or", "(ns b.logic)\n(defn -main [] (println (and true 1 2) (and true nil 2) (or nil false 9) (or nil false)))\n(-main)\n", "2 nil 9 false\n"),
        build("control-flow", "thread-first", "(ns b.thread)\n(defn -main [] (println (-> 10 inc inc) (-> 5 (* 2) (+ 1))))\n(-main)\n", "12 11\n"),
        // Level B — functions and closures.
        build("functions", "fixed-arity", "(ns b.fixed)\n(defn add [a b] (+ a b))\n(defn -main [] (println (add 1 2) (add -1 1) (add 20 22)))\n(-main)\n", "3 0 42\n"),
        build("functions", "direct-recursion", "(ns b.rec)\n(defn fib [n] (if (< n 2) n (+ (fib (dec n)) (fib (- n 2)))))\n(defn -main [] (println (fib 0) (fib 1) (fib 10)))\n(-main)\n", "0 1 55\n"),
        build("functions", "tail-recur", "(ns b.tail)\n(defn count-down [n acc] (if (= n 0) acc (recur (dec n) (inc acc))))\n(defn -main [] (println (count-down 0 0) (count-down 10 0) (count-down 10000 0)))\n(-main)\n", "0 10 10000\n"),
        build("functions", "multi-arity", "(ns b.multi)\n(defn greet ([] \"hi\") ([x] (str \"hi \" x)) ([a b] (str a \" \" b)))\n(defn -main [] (println (greet) (greet \"x\") (greet \"a\" \"b\")))\n(-main)\n", "hi hi x a b\n"),
        build("functions", "variadic", "(ns b.var)\n(defn collect [x & xs] (cons x xs))\n(defn -main [] (println (collect 1) (collect 1 2) (collect 1 2 3 4)))\n(-main)\n", "(1) (1 2) (1 2 3 4)\n"),
        build("functions", "apply", "(ns b.apply)\n(defn add3 [a b c] (+ a b c))\n(defn -main [] (println (apply add3 (list 1 2 3)) (apply add3 10 (list 20 30)) (apply add3 [7 8 9])))\n(-main)\n", "6 60 24\n"),
        build("functions", "higher-order", "(ns b.hof)\n(defn invoke [f x] (f x))\n(defn -main [] (println (invoke inc 1) (invoke (fn [x] (* x x)) 4) ((comp inc inc) 5)))\n(-main)\n", "2 16 7\n"),
        build("functions", "anonymous", "(ns b.anon)\n(defn -main [] (println ((fn [x] (+ x 1)) 4) ((fn [x y] (* x y)) 6 7)))\n(-main)\n", "5 42\n"),
        build("closures", "capture", "(ns b.capture)\n(defn -main [] (let [n 5 f (fn [x] (+ x n))] (println (f 0) (f 5) (f -5))))\n(-main)\n", "5 10 0\n"),
        build("closures", "factory", "(ns b.factory)\n(defn adder [n] (fn [x] (+ n x)))\n(defn -main [] (println ((adder 2) 3) ((adder -1) 1) ((adder 20) 22)))\n(-main)\n", "5 0 42\n"),
        build("closures", "nested", "(ns b.nested)\n(defn -main [] (println (((fn [a] (fn [b] (+ a b))) 3) 4)))\n(-main)\n", "7\n"),
        build_gc("closures", "capture-gc-stress", "(ns b.capture-gc)\n(defn make [x] (fn [y] (list x y)))\n(defn -main [] (println ((make (list 1 2)) (list 3 4))))\n(-main)\n", "((1 2) (3 4))\n"),
        // Level B — macros.
        build("macros", "current-core-macros", "(ns b.macros)\n(defn -main [] (when (and true true) (println (cond false 1 :else (or nil 42)))))\n(-main)\n", "42\n"),
        build_xfail("macros", "user-defmacro", "(ns b.user-macro)\n(defmacro twice [x] `(+ ~x ~x))\n(defn -main [] (println (twice 21)))\n(-main)\n", "42\n", "specs/LANGUAGE_SCOPE.md#macros"),
        // Level B — collections.
        build("collections", "lists", "(ns b.list)\n(defn -main [] (println (list) (list 1) (cons 1 (list 2 3)) (first (list 9)) (rest (list 9))))\n(-main)\n", "() (1) (1 2 3) 9 ()\n"),
        build("collections", "vectors", "(ns b.vec)\n(defn -main [] (println [1 2 3] (nth [10 20 30] 2) (assoc [1 2 3] 1 9) (conj [] 1)))\n(-main)\n", "[1 2 3] 30 [1 9 3] [1]\n"),
        build("collections", "array-map", "(ns b.map)\n(defn -main [] (let [m {:a 1 :b 2}] (println (get m :a) (:b m) (contains? m :c) (assoc m :c 3) (dissoc m :a))))\n(-main)\n", "1 2 false {:a 1, :b 2, :c 3} {:b 2}\n"),
        build("collections", "hash-map", "(ns b.hmap)\n(defn make [n] (loop [i 0 m {}] (if (< i n) (recur (inc i) (assoc m i (* i i))) m)))\n(defn -main [] (let [m (make 100)] (println (count m) (get m 99) (contains? m 50) (count (dissoc m 50)))))\n(-main)\n", "100 9801 true 99\n"),
        build("collections", "sets", "(ns b.set)\n(defn -main [] (println #{1 2 1} (count #{}) (contains? #{1 2} 2) (conj #{1 2} 3)))\n(-main)\n", "#{1 2} 0 true #{1 2 3}\n"),
        build("collections", "structural-equality", "(ns b.eq)\n(defn -main [] (println (= [1 2] [1 2]) (= {:a 1 :b 2} {:b 2 :a 1}) (= #{1 2} #{2 1}) (= nil false)))\n(-main)\n", "true true true false\n"),
        build("collections", "sequence-ops", "(ns b.seq)\n(defn -main [] (println (first [1 2]) (rest [1 2]) (empty? []) (count \"abc\") (count nil)))\n(-main)\n", "1 (2) true 3 0\n"),
        // Level B — records/protocols.
        build_expected_diff("records-protocols", "record", "(ns b.record)\n(defrecord Point [x y])\n(defn -main [] (let [p (->Point 3 4)] (println p (:x p) (assoc p :x 9) (= p (->Point 3 4)))))\n(-main)\n", "#Point{:x 3, :y 4} 3 #Point{:x 9, :y 4} true\n", "Native record printing omits the namespace qualifier that Clojure/JVM includes."),
        build("records-protocols", "protocol-record", "(ns b.proto)\n(defprotocol Shape (area [this]))\n(defrecord Rect [w h])\n(extend-type Rect Shape (area [this] (* (:w this) (:h this))))\n(defn -main [] (println (area (->Rect 3 4)) (area (->Rect 0 9))))\n(-main)\n", "12 0\n"),
        build("records-protocols", "extend-list", "(ns b.proto-list)\n(defprotocol Sized (size-of [this]))\n(extend-type List Sized (size-of [this] (count this)))\n(defn -main [] (println (size-of (list)) (size-of (list 1 2 3))))\n(-main)\n", "0 3\n"),
        // Level B — errors.
        build_error("unresolved-symbol", "(ns b.err)\n(defn -main [] (println missing))\n(-main)\n", "E0101"),
        build_error("bad-call-arity", "(ns b.err)\n(defn f [x] x)\n(defn -main [] (println (f 1 2)))\n(-main)\n", "E0103"),
        build_error("let-not-vector", "(ns b.err)\n(defn -main [] (let (:a 1) 1))\n(-main)\n", "E0104"),
        build_error("loop-odd-bindings", "(ns b.err)\n(defn -main [] (loop [a 1 b] a))\n(-main)\n", "E0106"),
        build_error("recur-outside", "(ns b.err)\n(recur 1)\n", "E0107"),
        build_error("recur-non-tail", "(ns b.err)\n(defn -main [] (+ 1 (recur)))\n(-main)\n", "E0108"),
        build_error("recur-bad-arity", "(ns b.err)\n(defn -main [] (loop [a 1] (recur a a)))\n(-main)\n", "E0109"),
        // Level B — GC.
        build_gc("gc", "list-rooting", "(ns b.gc-list)\n(defn make [n acc] (if (< n 0) acc (recur (dec n) (cons n acc))))\n(defn -main [] (println (count (make 100 (list))) (first (make 100 (list)))))\n(-main)\n", "101 0\n"),
        build_gc("gc", "collection-rooting", "(ns b.gc-coll)\n(defn -main [] (println (map (fn [x] (vector x (inc x))) (range 20))))\n(-main)\n", "([0 1] [1 2] [2 3] [3 4] [4 5] [5 6] [6 7] [7 8] [8 9] [9 10] [10 11] [11 12] [12 13] [13 14] [14 15] [15 16] [16 17] [17 18] [18 19] [19 20])\n"),
        build_gc("gc", "loop-garbage", "(ns b.gc-loop)\n(defn burn [n] (loop [i 0] (if (< i n) (do (count (cons i (list))) (recur (inc i))) i)))\n(defn -main [] (println (burn 20000)))\n(-main)\n", "20000\n"),
        // Level C — current embedded clojure.core, three scenarios each.
        core("zero-q", "(ns c.zero)\n(defn -main [] (println (zero? 0) (zero? 1) (zero? -1)))\n(-main)\n", "true false false\n"),
        core("pos-q", "(ns c.pos)\n(defn -main [] (println (pos? 1) (pos? 0) (pos? -1)))\n(-main)\n", "true false false\n"),
        core("neg-q", "(ns c.neg)\n(defn -main [] (println (neg? -1) (neg? 0) (neg? 1)))\n(-main)\n", "true false false\n"),
        core("even-q", "(ns c.even)\n(defn -main [] (println (even? 0) (even? -2) (even? 7)))\n(-main)\n", "true true false\n"),
        core("odd-q", "(ns c.odd)\n(defn -main [] (println (odd? 1) (odd? -3) (odd? 0)))\n(-main)\n", "true true false\n"),
        core("max", "(ns c.max)\n(defn -main [] (println (max 1 2 3) (max -5 -1) (max 7)))\n(-main)\n", "3 -1 7\n"),
        core("min", "(ns c.min)\n(defn -main [] (println (min 1 2 3) (min -5 -1) (min 7)))\n(-main)\n", "1 -5 7\n"),
        core("reduce", "(ns c.reduce)\n(defn -main [] (println (reduce + 0 (list 1 2 3)) (reduce + 10 (list)) (reduce * 1 [1 2 3 4])))\n(-main)\n", "6 10 24\n"),
        core("map", "(ns c.map)\n(defn -main [] (println (map inc (list 1 2 3)) (map inc (list)) (map (fn [x] (* x x)) [2 3])))\n(-main)\n", "(2 3 4) () (4 9)\n"),
        core("filter", "(ns c.filter)\n(defn -main [] (println (filter even? (range 6)) (filter even? (list)) (filter (fn [x] (> x 2)) (list 1 3 4))))\n(-main)\n", "(0 2 4) () (3 4)\n"),
        core("remove", "(ns c.remove)\n(defn -main [] (println (remove even? (range 6)) (remove even? (list)) (remove neg? (list -1 0 1))))\n(-main)\n", "(1 3 5) () (0 1)\n"),
        core("reverse", "(ns c.reverse)\n(defn -main [] (println (reverse (list 1 2 3)) (reverse (list)) (reverse (list :a))))\n(-main)\n", "(3 2 1) () (:a)\n"),
        core("take", "(ns c.take)\n(defn -main [] (println (take 2 (range 5)) (take 0 (range 5)) (take 9 (list 1 2))))\n(-main)\n", "(0 1) () (1 2)\n"),
        core("drop", "(ns c.drop)\n(defn -main [] (println (drop 2 (range 5)) (drop 0 (range 3)) (drop 9 (list 1 2))))\n(-main)\n", "(2 3 4) (0 1 2) ()\n"),
        core("range", "(ns c.range)\n(defn -main [] (println (range 5) (range 0) (range -2)))\n(-main)\n", "(0 1 2 3 4) () ()\n"),
        core("into", "(ns c.into)\n(defn -main [] (println (into [] (list 1 2)) (into (list) (list 1 2)) (into #{} (list 1 1 2))))\n(-main)\n", "[1 2] (2 1) #{1 2}\n"),
        core("mapv", "(ns c.mapv)\n(defn -main [] (println (mapv inc (range 3)) (mapv inc (list)) (mapv (fn [x] (* x 2)) (list -1 0 1))))\n(-main)\n", "[1 2 3] [] [-2 0 2]\n"),
        core("every-q", "(ns c.every)\n(defn -main [] (println (every? even? (list 2 4)) (every? even? (list)) (every? even? (list 2 3))))\n(-main)\n", "true true false\n"),
        core("some", "(ns c.some)\n(defn -main [] (println (some even? (list 1 2 3)) (some even? (list 1 3)) (some (fn [x] (if (> x 2) x nil)) (list 1 3 4))))\n(-main)\n", "true nil 3\n"),
        core("comp", "(ns c.comp)\n(defn -main [] (println ((comp inc inc) 0) ((comp dec inc) 9) ((comp (fn [x] (* x 2)) inc) 4)))\n(-main)\n", "2 9 10\n"),
        core("identity", "(ns c.identity)\n(defn -main [] (println (identity 42) (identity nil) (identity [:a :b])))\n(-main)\n", "42 nil [:a :b]\n"),
        core("second", "(ns c.second)\n(defn -main [] (println (second (list 1 2 3)) (second (list 1)) (second [4 5])))\n(-main)\n", "2 nil 5\n"),
        core("last", "(ns c.last)\n(defn -main [] (println (last (list 1 2 3)) (last (list 9)) (last [4 5])))\n(-main)\n", "3 9 5\n"),
        core("concat", "(ns c.concat)\n(defn -main [] (println (concat (list 1 2) (list 3 4)) (concat (list) (list 1)) (concat (list :a) (list))))\n(-main)\n", "(1 2 3 4) (1) (:a)\n"),
        core("mapcat", "(ns c.mapcat)\n(defn pair [x] (list x x))\n(defn -main [] (println (mapcat pair (list 1 2)) (mapcat pair (list)) (mapcat (fn [x] (list (inc x))) (list 0 1))))\n(-main)\n", "(1 1 2 2) () (1 2)\n"),
        core("count-if", "(ns c.count-if)\n(defn -main [] (println (count-if even? (range 6)) (count-if even? (list)) (count-if neg? (list -2 -1 0 1))))\n(-main)\n", "3 0 2\n"),
    ];

    // Every active clojure.core function group also contains an explicit invalid
    // call. Together with the three calls above, this covers normal, boundary,
    // alternate-input, and error scenarios.
    for (slug, function) in [
        ("zero-q", "zero?"),
        ("pos-q", "pos?"),
        ("neg-q", "neg?"),
        ("even-q", "even?"),
        ("odd-q", "odd?"),
        ("max", "max"),
        ("min", "min"),
        ("reduce", "reduce"),
        ("map", "map"),
        ("filter", "filter"),
        ("remove", "remove"),
        ("reverse", "reverse"),
        ("take", "take"),
        ("drop", "drop"),
        ("range", "range"),
        ("into", "into"),
        ("mapv", "mapv"),
        ("every-q", "every?"),
        ("some", "some"),
        ("comp", "comp"),
        ("identity", "identity"),
        ("second", "second"),
        ("last", "last"),
        ("concat", "concat"),
        ("mapcat", "mapcat"),
        ("count-if", "count-if"),
    ] {
        cases.push(core_invalid_arity(slug, function));
    }

    // Documented but not currently loadable standard-library namespaces.
    for (directory, namespace, function, input) in [
        (
            "clojure-string",
            "clojure.string",
            "join",
            "(clojure.string/join \",\" [\"a\" \"b\"])\n",
        ),
        (
            "clojure-string",
            "clojure.string",
            "split",
            "(clojure.string/split \"a,b\" \",\")\n",
        ),
        (
            "clojure-string",
            "clojure.string",
            "trim",
            "(clojure.string/trim \" a \")\n",
        ),
        (
            "clojure-string",
            "clojure.string",
            "upper-case",
            "(clojure.string/upper-case \"abc\")\n",
        ),
        (
            "clojure-string",
            "clojure.string",
            "lower-case",
            "(clojure.string/lower-case \"ABC\")\n",
        ),
        (
            "clojure-string",
            "clojure.string",
            "replace",
            "(clojure.string/replace \"aba\" \"a\" \"x\")\n",
        ),
        (
            "clojure-string",
            "clojure.string",
            "starts-with?",
            "(clojure.string/starts-with? \"abc\" \"a\")\n",
        ),
        (
            "clojure-set",
            "clojure.set",
            "union",
            "(clojure.set/union #{1} #{2})\n",
        ),
        (
            "clojure-set",
            "clojure.set",
            "intersection",
            "(clojure.set/intersection #{1 2} #{2 3})\n",
        ),
        (
            "clojure-set",
            "clojure.set",
            "difference",
            "(clojure.set/difference #{1 2} #{2})\n",
        ),
        (
            "clojure-set",
            "clojure.set",
            "subset?",
            "(clojure.set/subset? #{1} #{1 2})\n",
        ),
        (
            "clojure-walk",
            "clojure.walk",
            "walk",
            "(clojure.walk/walk identity identity [1 2])\n",
        ),
        (
            "clojure-walk",
            "clojure.walk",
            "postwalk",
            "(clojure.walk/postwalk identity {:a 1})\n",
        ),
        (
            "clojure-walk",
            "clojure.walk",
            "prewalk",
            "(clojure.walk/prewalk identity {:a 1})\n",
        ),
        (
            "clojure-walk",
            "clojure.walk",
            "macroexpand-all",
            "(clojure.walk/macroexpand-all '(when true 1))\n",
        ),
        (
            "clojure-edn",
            "clojure.edn",
            "read-string",
            "(clojure.edn/read-string \"{:a 1}\")\n",
        ),
        (
            "clojure-test",
            "clojure.test",
            "deftest",
            "(clojure.test/deftest sample (clojure.test/is true))\n",
        ),
        (
            "clojure-test",
            "clojure.test",
            "is",
            "(clojure.test/is (= 1 1))\n",
        ),
        (
            "clojure-test",
            "clojure.test",
            "are",
            "(clojure.test/are [x] (pos? x) 1 2)\n",
        ),
        (
            "clojure-test",
            "clojure.test",
            "testing",
            "(clojure.test/testing \"sample\" (clojure.test/is true))\n",
        ),
        (
            "clojure-test",
            "clojure.test",
            "run-tests",
            "(clojure.test/run-tests)\n",
        ),
    ] {
        cases.push(pending_stdlib(directory, namespace, function, input));
    }

    // Level D — pure-library project inventory.
    cases.extend([
        pending_project('D', "functional-library", "pure-libraries/functional", "Requires a multi-file project loader for a pure functional library.", "(ns fixture.functional)\n(defn transform [xs] (map inc xs))\n"),
        pending_project('D', "macro-library", "pure-libraries/macros", "Requires build-time loading of macros from another namespace.", "(ns fixture.macros)\n(defmacro unless [p x] `(if ~p nil ~x))\n"),
        pending_project('D', "multiple-namespaces", "pure-libraries/namespaces", "Requires dependency ordering across multiple source files.", "(ns fixture.app (:require [fixture.lib :as lib]))\n(lib/value)\n"),
        pending_project('D', "protocol-record-library", "pure-libraries/protocols-records", "Requires project-level protocol and record definitions.", "(ns fixture.protocols)\n(defprotocol P (value [x]))\n"),
        pending_project('D', "persistent-structure-library", "pure-libraries/persistent-structures", "Requires a reusable project that builds a persistent data structure.", "(ns fixture.tree)\n(defn node [v l r] {:value v :left l :right r})\n"),
        // Level E — ecosystem inventory.
        pending_project('E', "external-dependency", "ecosystem/dependencies", "No dependency resolver exists in the native project path.", "(ns fixture.external (:require [external.lib :as ext]))\n(ext/run)\n"),
        pending_project('E', "jar-classpath", "ecosystem/jar-classpath", "JAR and JVM classpath loading are outside the native runtime.", "(ns fixture.jar)\n"),
        pending_project('E', "java-interop", "ecosystem/java-interop", "Java interop has no native equivalent.", "(ns fixture.java)\n(System/currentTimeMillis)\n"),
        pending_project('E', "dynamic-loading", "ecosystem/dynamic-loading", "Dynamic require and eval are not part of the current AOT path.", "(ns fixture.dynamic)\n(require 'fixture.other)\n"),
        pending_project('E', "integrated-application", "ecosystem/application", "An integrated application needs modules, resources, dependencies, and stable packaging.", "(ns fixture.application)\n(defn -main [] (println \"app\"))\n"),
    ]);

    cases
}
