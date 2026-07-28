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
    expected_stderr: Option<&'static str>,
    extra_files: &'static [(&'static str, &'static str)],
    extra_binary_files: &'static [(&'static str, &'static [u8])],
    manifest_tail: &'static str,
}

const PEDESTAL_HELLO_SOURCE: &str = "(ns hello
  (:require [io.pedestal.connector :as conn]
            [io.pedestal.http.http-kit :as hk]))

(defn greet-handler [_request]
  {:status 200
   :body \"Hello, world!\\n\"})

(def routes
  #{[\"/greet\" :get greet-handler :route-name :greet]})

(defn create-connector []
  (-> (conn/default-connector-map 8890)
      (conn/with-default-interceptors)
      (conn/with-routes routes)
      (hk/create-connector nil)))

(defn start []
  (conn/start! (create-connector)))

(defn -main [& _args]
  (start))
";

const PEDESTAL_PROJECT_FILES: &[(&str, &str)] = &[
    (
        "deps.edn",
        r#"{:paths ["src"]
 :deps {io.pedestal/pedestal.http-kit {:mvn/version "0.8.2-beta-10"}
        org.slf4j/slf4j-simple {:mvn/version "2.0.17"}}
 :aliases {:run {:main-opts ["-m" "hello"]}}}
"#,
    ),
    ("src/hello.clj", PEDESTAL_HELLO_SOURCE),
    (
        "request.http",
        "GET /greet HTTP/1.1\nHost: localhost:8890\n",
    ),
    (
        "expected-response.edn",
        "{:status 200 :body \"Hello, world!\\n\"}\n",
    ),
];

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
    if directory.exists() {
        for generated in [
            "case.toml",
            "input.clj",
            "expected.edn",
            "expected.stdout",
            "expected.stdout.bin",
            "expected.stderr",
            "expected.stderr.bin",
            "work.before",
            "work.after",
            "stdin.txt",
            "stdin-crlf.bin",
            "stdin-invalid.bin",
        ] {
            let path = directory.join(generated);
            if path.is_dir() {
                fs::remove_dir_all(path).expect("reset generated fixture directory");
            } else if path.exists() {
                fs::remove_file(path).expect("reset generated fixture file");
            }
        }
    }
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
         {}\
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
        namespace,
        fixture.manifest_tail
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
    if let Some(value) = fixture.expected_stderr {
        fs::write(directory.join("expected.stderr"), value).expect("write expected.stderr");
    }
    for (relative_path, contents) in fixture.extra_files {
        let path = directory.join(relative_path);
        fs::create_dir_all(path.parent().expect("extra fixture file parent"))
            .expect("create extra fixture directory");
        fs::write(path, contents).expect("write extra fixture file");
    }
    for (relative_path, contents) in fixture.extra_binary_files {
        let path = directory.join(relative_path);
        fs::create_dir_all(path.parent().expect("extra binary fixture file parent"))
            .expect("create extra binary fixture directory");
        fs::write(path, contents).expect("write extra binary fixture file");
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
        expected_stderr: None,
        extra_files: &[],
        extra_binary_files: &[],
        manifest_tail: "",
    }
}

fn with_extra_files(
    mut fixture: Fixture,
    extra_files: &'static [(&'static str, &'static str)],
) -> Fixture {
    fixture.extra_files = extra_files;
    fixture
}

fn with_binary_files(
    mut fixture: Fixture,
    extra_binary_files: &'static [(&'static str, &'static [u8])],
) -> Fixture {
    fixture.extra_binary_files = extra_binary_files;
    fixture
}

fn with_run(mut fixture: Fixture, manifest_tail: &'static str) -> Fixture {
    fixture.manifest_tail = manifest_tail;
    fixture
}

fn with_expected_stderr(mut fixture: Fixture, expected_stderr: &'static str) -> Fixture {
    fixture.expected_stderr = Some(expected_stderr);
    fixture
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
        leak(format!("b.{}.{}", sanitize(area), slug.replace('-', "_"))),
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
        leak(format!("b.{}.{}", sanitize(area), slug.replace('-', "_"))),
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

#[allow(clippy::too_many_arguments)]
fn build_xfail_reason(
    area: &'static str,
    slug: &'static str,
    body: &'static str,
    desired: &'static str,
    reason: &'static str,
    tracking: &'static str,
) -> Fixture {
    let mut value = build_xfail(area, slug, body, desired, tracking);
    value.reason = reason;
    value
}

fn native_build_xfail(
    area: &'static str,
    slug: &'static str,
    body: &'static str,
    desired: &'static str,
) -> Fixture {
    let mut value = build_xfail(area, slug, body, desired, "specs/IO_SPEC.md");
    value.oracle = "not-applicable";
    value
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

fn pedestal_hello_world_project() -> Fixture {
    with_extra_files(
        fixture(
            'E',
            "web-frameworks/pedestal",
            "hello-world-api",
            "e.pedestal.hello_world_api",
            "ecosystem/web-frameworks/pedestal",
            "pending",
            "unsupported",
            "project",
            "not-applicable",
            false,
            "Requires Maven dependency resolution, multi-file namespaces, Pedestal, HTTP-Kit, networking, and a managed server lifecycle.",
            "specs/COMPATIBILITY_SPEC.md",
            Some("hello"),
            PEDESTAL_HELLO_SOURCE,
            Expected::None,
        ),
        PEDESTAL_PROJECT_FILES,
    )
}

#[allow(clippy::too_many_arguments)]
fn higher_level_build(
    level: char,
    area_directory: &'static str,
    slug: &'static str,
    area: &'static str,
    body: &'static str,
    expected: &'static str,
    gc_stress: bool,
) -> Fixture {
    let reason = match level {
        'D' => "This self-contained pure-library slice is executable on the current native path.",
        'E' => "This self-contained application slice builds and runs as a standalone native executable.",
        _ => panic!("higher-level build requires level D or E"),
    };
    fixture(
        level,
        area_directory,
        slug,
        leak(format!(
            "{}.{}.{}",
            level.to_ascii_lowercase(),
            area_directory.replace('-', "_"),
            slug.replace('-', "_")
        )),
        area,
        "active",
        "official",
        "build-run",
        "equal",
        gc_stress,
        reason,
        "specs/COMPATIBILITY_SPEC.md",
        None,
        body,
        Expected::Stdout(expected),
    )
}

#[allow(clippy::too_many_arguments)]
fn higher_level_xfail(
    level: char,
    area_directory: &'static str,
    slug: &'static str,
    area: &'static str,
    body: &'static str,
    desired: &'static str,
    oracle: &'static str,
    reason: &'static str,
) -> Fixture {
    fixture(
        level,
        area_directory,
        slug,
        leak(format!(
            "{}.{}.{}",
            level.to_ascii_lowercase(),
            area_directory.replace('-', "_"),
            slug.replace('-', "_")
        )),
        area,
        "xfail",
        "unsupported",
        "build-run",
        oracle,
        false,
        reason,
        "specs/COMPATIBILITY_SPEC.md",
        None,
        body,
        Expected::Stdout(desired),
    )
}

#[derive(Clone, Copy)]
struct IoApi {
    namespace: &'static str,
    directory: &'static str,
    function: &'static str,
    normal: &'static str,
    boundary: &'static str,
    invalid: &'static str,
}

const IO_CASE_FILES: &[(&str, &str)] = &[
    ("work.before/input.txt", "alpha\nbeta\n"),
    ("work.before/empty.txt", ""),
    ("work.before/tree/a.txt", "a\n"),
    ("work.before/tree/nested/b.txt", "b\n"),
];

const SPIT_NORMAL_FILES: &[(&str, &str)] = &[("work.after/created.txt", "hello\n")];

const INVALID_UTF8_FILE: &[(&str, &[u8])] = &[("stdin-invalid.bin", &[0x66, 0x80, 0x6f])];

fn io_api_case(api: IoApi, scenario: &'static str, expression: &'static str) -> Fixture {
    let function_slug = sanitize(api.function);
    let namespace_slug = sanitize(api.namespace);
    let observed_expression = match (api.namespace, api.function, scenario) {
        ("clojure.core", "slurp", "normal") => {
            leak(format!("(= \"alpha\\nbeta\\n\" {expression})"))
        }
        ("clojure.core", "slurp", "boundary") | ("clojure.core", "with-out-str", "boundary") => {
            leak(format!("(= \"\" {expression})"))
        }
        ("clojure.core", "with-in-str", "normal") | ("clojure.core", "with-out-str", "normal") => {
            leak(format!("(= \"alpha\" {expression})"))
        }
        ("clojure.core", "read-string", "normal") => {
            leak(format!("(= {{:answer 42}} {expression})"))
        }
        ("clojure.core", "with-in-str", "boundary")
        | ("clojure.core", "read-string", "boundary") => leak(format!("(nil? {expression})")),
        _ => expression,
    };
    let observes_result = observed_expression != expression;
    let body = if scenario == "error" {
        leak(format!(
            "(ns io.{namespace_slug}.{function_slug}.{scenario})\n\
             (defn -main []\n\
               (try\n\
                 (do {expression} (println :unexpected))\n\
                 (catch cljn.io/IOException error\n\
                   (println (:kind (ex-data error))))))\n\
             (-main)\n"
        ))
    } else if observes_result {
        leak(format!(
            "(ns io.{namespace_slug}.{function_slug}.{scenario})\n\
             (defn -main [] (println {observed_expression}))\n\
             (-main)\n"
        ))
    } else {
        leak(format!(
            "(ns io.{namespace_slug}.{function_slug}.{scenario})\n\
             (defn -main [] (do {expression} (println :ok)))\n\
             (-main)\n"
        ))
    };
    let expected = if scenario == "error" {
        ":invalid-input\n"
    } else if observes_result {
        "true\n"
    } else {
        ":ok\n"
    };
    let oracle = if matches!(api.namespace, "clojure.core" | "clojure.edn") {
        "equal"
    } else {
        "not-applicable"
    };
    let class = if oracle == "equal" {
        "official"
    } else {
        "unsupported"
    };
    let mut value = fixture(
        'C',
        leak(format!("{}/{}", api.directory, function_slug)),
        scenario,
        leak(format!(
            "c.{}.{}.{}",
            namespace_slug, function_slug, scenario
        )),
        leak(format!("stdlib/{}", api.directory)),
        "xfail",
        class,
        "build-run",
        oracle,
        scenario == "boundary",
        "The I/O contract is specified and executable as a fixture, but its native API is not implemented yet.",
        "specs/IO_SPEC.md",
        Some(api.namespace),
        body,
        Expected::Stdout(expected),
    );
    if ["input.txt", "empty.txt", "\"tree", "\".\""]
        .iter()
        .any(|needle| expression.contains(needle))
    {
        value.extra_files = IO_CASE_FILES;
    }
    if api.namespace == "clojure.core" && api.function == "spit" && scenario == "normal" {
        value.extra_files = SPIT_NORMAL_FILES;
    }
    if !matches!(api.namespace, "clojure.core" | "clojure.edn") {
        value.manifest_tail = "[run]\nplatforms = [\"linux\"]\n";
    }
    value
}

fn io_api_cases() -> Vec<Fixture> {
    let apis = [
        // clojure.core I/O and dynamic redirection.
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io",
            function: "pr",
            normal: "(pr {:answer 42})",
            boundary: "(pr nil)",
            invalid: "(binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (pr 1))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io",
            function: "prn",
            normal: "(prn \"line\")",
            boundary: "(prn)",
            invalid: "(binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (prn 1))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io",
            function: "newline",
            normal: "(newline)",
            boundary: "(binding [*flush-on-newline* false] (newline))",
            invalid: "(binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (newline))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io",
            function: "flush",
            normal: "(flush)",
            boundary: "(binding [*out* (cljn.io/string-writer)] (flush))",
            invalid: "(binding [*out* (doto (cljn.io/string-writer) cljn.io/close!)] (flush))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io",
            function: "read-line",
            normal: "(binding [*in* (cljn.io/string-reader \"alpha\\nbeta\\n\")] (read-line))",
            boundary: "(binding [*in* (cljn.io/string-reader \"\")] (read-line))",
            invalid: "(binding [*in* (cljn.io/byte-input-stream (cljn.io/bytes [128]))] (read-line))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io",
            function: "slurp",
            normal: "(slurp \"input.txt\")",
            boundary: "(slurp \"empty.txt\")",
            invalid: "(slurp \"missing.txt\")",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io",
            function: "spit",
            normal: "(spit \"created.txt\" \"hello\\n\")",
            boundary: "(spit \"created.txt\" \"\" :append true)",
            invalid: "(spit \"tree\" \"not-a-file\")",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/reader",
            function: "read",
            normal: "(read (cljn.io/string-reader \"{:answer 42}\"))",
            boundary: "(read {:eof :done} (cljn.io/string-reader \"\"))",
            invalid: "(read (cljn.io/string-reader \"[1\"))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/reader",
            function: "read-string",
            normal: "(read-string \"{:answer 42}\")",
            boundary: "(read-string \"nil\")",
            invalid: "(read-string \"#=(+ 1 2)\")",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io-macros",
            function: "with-open",
            normal: "(with-open [r (cljn.io/string-reader \"x\")] (cljn.io/read-char r))",
            boundary: "(with-open [] nil)",
            invalid: "(with-open [r (cljn.io/string-reader \"x\")] (throw (ex-info \"boom\" {})))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io-macros",
            function: "with-in-str",
            normal: "(with-in-str \"alpha\\n\" (read-line))",
            boundary: "(with-in-str \"\" (read-line))",
            invalid: "(with-in-str nil (read-line))",
        },
        IoApi {
            namespace: "clojure.core",
            directory: "clojure-core/io-macros",
            function: "with-out-str",
            normal: "(with-out-str (print \"alpha\"))",
            boundary: "(with-out-str nil)",
            invalid: "(with-out-str (throw (ex-info \"boom\" {})))",
        },
        // Runtime EDN reader.
        IoApi {
            namespace: "clojure.edn",
            directory: "clojure-edn",
            function: "read",
            normal: "(clojure.edn/read (cljn.io/string-reader \"{:a [1 2]}\"))",
            boundary: "(clojure.edn/read {:eof :done} (cljn.io/string-reader \"\"))",
            invalid: "(clojure.edn/read (cljn.io/string-reader \"{:a\"))",
        },
        IoApi {
            namespace: "clojure.edn",
            directory: "clojure-edn",
            function: "read-string",
            normal: "(clojure.edn/read-string \"#{1 2}\")",
            boundary: "(clojure.edn/read-string {:readers {} :default (fn [tag value] [tag value])} \"#app/id 42\")",
            invalid: "(clojure.edn/read-string \"#=(+ 1 2)\")",
        },
        // Native paths.
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/paths",
            function: "path",
            normal: "(cljn.io/path \"tree/nested/b.txt\")",
            boundary: "(cljn.io/path \"\")",
            invalid: "(cljn.io/path nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/paths",
            function: "join",
            normal: "(cljn.io/join (cljn.io/path \"tree\") \"nested\" \"b.txt\")",
            boundary: "(cljn.io/join (cljn.io/path \".\"))",
            invalid: "(cljn.io/join nil \"x\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/paths",
            function: "parent",
            normal: "(cljn.io/parent (cljn.io/path \"tree/a.txt\"))",
            boundary: "(cljn.io/parent (cljn.io/path \"/\"))",
            invalid: "(cljn.io/parent \"not-a-path\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/paths",
            function: "file-name",
            normal: "(cljn.io/file-name (cljn.io/path \"tree/a.txt\"))",
            boundary: "(cljn.io/file-name (cljn.io/path \"/\"))",
            invalid: "(cljn.io/file-name nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/paths",
            function: "normalize",
            normal: "(cljn.io/normalize (cljn.io/path \"tree/../input.txt\"))",
            boundary: "(cljn.io/normalize (cljn.io/path \".\"))",
            invalid: "(cljn.io/normalize 42)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/paths",
            function: "real-path",
            normal: "(cljn.io/real-path (cljn.io/path \"input.txt\"))",
            boundary: "(cljn.io/real-path (cljn.io/path \".\"))",
            invalid: "(cljn.io/real-path (cljn.io/path \"missing\"))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/paths",
            function: "absolute?",
            normal: "(cljn.io/absolute? (cljn.io/path \"/tmp\"))",
            boundary: "(cljn.io/absolute? (cljn.io/path \"\"))",
            invalid: "(cljn.io/absolute? \"relative\")",
        },
        // Immutable bytes.
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/bytes",
            function: "bytes",
            normal: "(cljn.io/bytes [0 1 255])",
            boundary: "(cljn.io/bytes [])",
            invalid: "(cljn.io/bytes [256])",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/bytes",
            function: "bytes?",
            normal: "(cljn.io/bytes? (cljn.io/bytes [1]))",
            boundary: "(cljn.io/bytes? nil)",
            invalid: "(cljn.io/bytes? (cljn.io/bytes [256]))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/bytes",
            function: "byte-count",
            normal: "(cljn.io/byte-count (cljn.io/bytes [1 2 3]))",
            boundary: "(cljn.io/byte-count (cljn.io/bytes []))",
            invalid: "(cljn.io/byte-count \"abc\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/bytes",
            function: "bytes->vector",
            normal: "(cljn.io/bytes->vector (cljn.io/bytes [0 127 255]))",
            boundary: "(cljn.io/bytes->vector (cljn.io/bytes []))",
            invalid: "(cljn.io/bytes->vector nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/bytes",
            function: "bytes->string",
            normal: "(cljn.io/bytes->string (cljn.io/bytes [97 195 167 195 163 111]))",
            boundary: "(cljn.io/bytes->string (cljn.io/bytes []))",
            invalid: "(cljn.io/bytes->string (cljn.io/bytes [128]))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/bytes",
            function: "string->bytes",
            normal: "(cljn.io/string->bytes \"ação\")",
            boundary: "(cljn.io/string->bytes \"\")",
            invalid: "(cljn.io/string->bytes nil)",
        },
        // Handles, buffered character I/O, and binary I/O.
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "reader",
            normal: "(cljn.io/close! (cljn.io/reader \"input.txt\"))",
            boundary: "(cljn.io/close! (cljn.io/reader \"empty.txt\"))",
            invalid: "(cljn.io/reader \"missing.txt\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "writer",
            normal: "(cljn.io/close! (cljn.io/writer \"created.txt\"))",
            boundary: "(cljn.io/close! (cljn.io/writer \"created.txt\" :append true))",
            invalid: "(cljn.io/writer \"tree\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "input-stream",
            normal: "(cljn.io/close! (cljn.io/input-stream \"input.txt\"))",
            boundary: "(cljn.io/close! (cljn.io/input-stream \"empty.txt\"))",
            invalid: "(cljn.io/input-stream \"missing.txt\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "output-stream",
            normal: "(cljn.io/close! (cljn.io/output-stream \"created.bin\"))",
            boundary: "(cljn.io/close! (cljn.io/output-stream \"created.bin\" :append true))",
            invalid: "(cljn.io/output-stream \"tree\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "string-reader",
            normal: "(cljn.io/close! (cljn.io/string-reader \"abc\"))",
            boundary: "(cljn.io/close! (cljn.io/string-reader \"\"))",
            invalid: "(cljn.io/string-reader nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "string-writer",
            normal: "(cljn.io/close! (cljn.io/string-writer))",
            boundary: "(cljn.io/string-writer 0)",
            invalid: "(cljn.io/string-writer -1)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "writer-string",
            normal: "(let [w (cljn.io/string-writer)] (cljn.io/write! w \"ação\") (cljn.io/writer-string w))",
            boundary: "(cljn.io/writer-string (cljn.io/string-writer))",
            invalid: "(cljn.io/writer-string (cljn.io/string-reader \"x\"))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "byte-input-stream",
            normal: "(cljn.io/close! (cljn.io/byte-input-stream (cljn.io/bytes [1 2])))",
            boundary: "(cljn.io/close! (cljn.io/byte-input-stream (cljn.io/bytes [])))",
            invalid: "(cljn.io/byte-input-stream \"bytes\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "byte-output-stream",
            normal: "(cljn.io/close! (cljn.io/byte-output-stream))",
            boundary: "(cljn.io/byte-output-stream 0)",
            invalid: "(cljn.io/byte-output-stream -1)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/resources",
            function: "output-bytes",
            normal: "(let [out (cljn.io/byte-output-stream)] (cljn.io/write-bytes! out (cljn.io/bytes [0 255])) (cljn.io/output-bytes out))",
            boundary: "(cljn.io/output-bytes (cljn.io/byte-output-stream))",
            invalid: "(cljn.io/output-bytes (cljn.io/input-stream \"input.txt\"))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/lifecycle",
            function: "close!",
            normal: "(cljn.io/close! (cljn.io/string-reader \"x\"))",
            boundary: "(let [r (cljn.io/string-reader \"x\")] (cljn.io/close! r) (cljn.io/close! r))",
            invalid: "(cljn.io/close! *out*)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/lifecycle",
            function: "closed?",
            normal: "(let [r (cljn.io/string-reader \"x\")] (cljn.io/close! r) (cljn.io/closed? r))",
            boundary: "(cljn.io/closed? (cljn.io/string-reader \"\"))",
            invalid: "(cljn.io/closed? 42)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/lifecycle",
            function: "flush!",
            normal: "(cljn.io/flush! (cljn.io/string-writer))",
            boundary: "(cljn.io/flush! *out*)",
            invalid: "(cljn.io/flush! (doto (cljn.io/string-writer) cljn.io/close!))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/text",
            function: "read-char",
            normal: "(cljn.io/read-char (cljn.io/string-reader \"λ\"))",
            boundary: "(cljn.io/read-char (cljn.io/string-reader \"\"))",
            invalid: "(cljn.io/read-char (doto (cljn.io/string-reader \"x\") cljn.io/close!))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/text",
            function: "unread-char",
            normal: "(let [r (cljn.io/string-reader \"ab\")] (cljn.io/unread-char r (cljn.io/read-char r)))",
            boundary: "(let [r (cljn.io/string-reader \"a\")] (cljn.io/read-char r) (cljn.io/unread-char r \\a))",
            invalid: "(cljn.io/unread-char (cljn.io/string-reader \"\") \\a)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/text",
            function: "read-line",
            normal: "(cljn.io/read-line (cljn.io/string-reader \"a\\r\\nb\\n\"))",
            boundary: "(cljn.io/read-line (cljn.io/string-reader \"last\"))",
            invalid: "(cljn.io/read-line (doto (cljn.io/string-reader \"x\") cljn.io/close!))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/text",
            function: "read-block!",
            normal: "(cljn.io/read-block! (cljn.io/string-reader \"abcd\") 4)",
            boundary: "(cljn.io/read-block! (cljn.io/string-reader \"\") 0)",
            invalid: "(cljn.io/read-block! (cljn.io/string-reader \"x\") -1)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/binary",
            function: "read-bytes",
            normal: "(cljn.io/read-bytes (cljn.io/byte-input-stream (cljn.io/bytes [0 255])) 2)",
            boundary: "(cljn.io/read-bytes (cljn.io/byte-input-stream (cljn.io/bytes [])) 0)",
            invalid: "(cljn.io/read-bytes (cljn.io/string-reader \"x\") 1)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/text",
            function: "write!",
            normal: "(cljn.io/write! (cljn.io/string-writer) \"ação\")",
            boundary: "(cljn.io/write! (cljn.io/string-writer) \"\")",
            invalid: "(cljn.io/write! (doto (cljn.io/string-writer) cljn.io/close!) \"x\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/binary",
            function: "write-bytes!",
            normal: "(cljn.io/write-bytes! (cljn.io/byte-output-stream) (cljn.io/bytes [0 255]))",
            boundary: "(cljn.io/write-bytes! (cljn.io/byte-output-stream) (cljn.io/bytes []))",
            invalid: "(cljn.io/write-bytes! (cljn.io/string-writer) (cljn.io/bytes [1]))",
        },
        // Positioning and filesystem.
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/files",
            function: "seek!",
            normal: "(with-open [s (cljn.io/input-stream \"input.txt\")] (cljn.io/seek! s 2))",
            boundary: "(with-open [s (cljn.io/input-stream \"empty.txt\")] (cljn.io/seek! s 0))",
            invalid: "(with-open [s (cljn.io/input-stream \"input.txt\")] (cljn.io/seek! s -1))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/files",
            function: "position",
            normal: "(with-open [s (cljn.io/input-stream \"input.txt\")] (cljn.io/position s))",
            boundary: "(with-open [s (cljn.io/input-stream \"empty.txt\")] (cljn.io/position s))",
            invalid: "(cljn.io/position (cljn.io/string-reader \"x\"))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/files",
            function: "truncate!",
            normal: "(with-open [s (cljn.io/output-stream \"created.bin\")] (cljn.io/truncate! s 2))",
            boundary: "(with-open [s (cljn.io/output-stream \"created.bin\")] (cljn.io/truncate! s 0))",
            invalid: "(with-open [s (cljn.io/output-stream \"created.bin\")] (cljn.io/truncate! s -1))",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/files",
            function: "copy!",
            normal: "(cljn.io/copy! \"input.txt\" \"copy.txt\")",
            boundary: "(cljn.io/copy! \"empty.txt\" \"empty-copy.txt\")",
            invalid: "(cljn.io/copy! \"missing.txt\" \"copy.txt\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "exists?",
            normal: "(cljn.io/exists? \"input.txt\")",
            boundary: "(cljn.io/exists? \"missing.txt\")",
            invalid: "(cljn.io/exists? nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "file?",
            normal: "(cljn.io/file? \"input.txt\")",
            boundary: "(cljn.io/file? \"tree\")",
            invalid: "(cljn.io/file? nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "directory?",
            normal: "(cljn.io/directory? \"tree\")",
            boundary: "(cljn.io/directory? \"input.txt\")",
            invalid: "(cljn.io/directory? nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "symlink?",
            normal: "(cljn.io/symlink? \"link.txt\")",
            boundary: "(cljn.io/symlink? \"input.txt\")",
            invalid: "(cljn.io/symlink? nil)",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "attributes",
            normal: "(cljn.io/attributes \"input.txt\")",
            boundary: "(cljn.io/attributes \"empty.txt\")",
            invalid: "(cljn.io/attributes \"missing.txt\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "list",
            normal: "(cljn.io/list \"tree\")",
            boundary: "(cljn.io/list \".\")",
            invalid: "(cljn.io/list \"input.txt\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "create-directory!",
            normal: "(cljn.io/create-directory! \"new-dir\")",
            boundary: "(cljn.io/create-directory! \"new-dir\" :exists :ignore)",
            invalid: "(cljn.io/create-directory! \"tree\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "create-directories!",
            normal: "(cljn.io/create-directories! \"new/a/b\")",
            boundary: "(cljn.io/create-directories! \".\")",
            invalid: "(cljn.io/create-directories! \"input.txt/child\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "move!",
            normal: "(cljn.io/move! \"input.txt\" \"moved.txt\")",
            boundary: "(cljn.io/move! \"empty.txt\" \"moved-empty.txt\")",
            invalid: "(cljn.io/move! \"missing.txt\" \"moved.txt\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "delete!",
            normal: "(cljn.io/delete! \"input.txt\")",
            boundary: "(cljn.io/delete! \"missing.txt\" :missing :ignore)",
            invalid: "(cljn.io/delete! \"tree\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "copy-tree!",
            normal: "(cljn.io/copy-tree! \"tree\" \"tree-copy\")",
            boundary: "(cljn.io/copy-tree! \"tree\" \"tree-copy\" :overwrite true)",
            invalid: "(cljn.io/copy-tree! \"/\" \"root-copy\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "delete-tree!",
            normal: "(cljn.io/delete-tree! \"tree\")",
            boundary: "(cljn.io/delete-tree! \"missing\" :missing :ignore)",
            invalid: "(cljn.io/delete-tree! \"/\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "create-symlink!",
            normal: "(cljn.io/create-symlink! \"input.txt\" \"link.txt\")",
            boundary: "(cljn.io/create-symlink! \"missing-target\" \"dangling\")",
            invalid: "(cljn.io/create-symlink! \"input.txt\" \"input.txt\")",
        },
        IoApi {
            namespace: "cljn.io",
            directory: "cljn-io/filesystem",
            function: "read-link",
            normal: "(do (cljn.io/create-symlink! \"input.txt\" \"link.txt\") (cljn.io/read-link \"link.txt\"))",
            boundary: "(do (cljn.io/create-symlink! \"missing\" \"dangling\") (cljn.io/read-link \"dangling\"))",
            invalid: "(cljn.io/read-link \"input.txt\")",
        },
        // Process context.
        IoApi {
            namespace: "cljn.process",
            directory: "cljn-process",
            function: "getenv",
            normal: "(cljn.process/getenv \"CLJN_CONFORMANCE_VALUE\")",
            boundary: "(cljn.process/getenv \"CLJN_CONFORMANCE_MISSING\")",
            invalid: "(cljn.process/getenv nil)",
        },
        IoApi {
            namespace: "cljn.process",
            directory: "cljn-process",
            function: "environment",
            normal: "(cljn.process/environment)",
            boundary: "(contains? (cljn.process/environment) \"CLJN_CONFORMANCE_VALUE\")",
            invalid: "(cljn.process/environment :unexpected)",
        },
        IoApi {
            namespace: "cljn.process",
            directory: "cljn-process",
            function: "cwd",
            normal: "(cljn.process/cwd)",
            boundary: "(cljn.io/absolute? (cljn.process/cwd))",
            invalid: "(cljn.process/cwd :unexpected)",
        },
    ];

    let mut cases = Vec::with_capacity(apis.len() * 3);
    for api in apis {
        cases.push(io_api_case(api, "normal", api.normal));
        cases.push(io_api_case(api, "boundary", api.boundary));
        cases.push(io_api_case(api, "error", api.invalid));
    }
    cases
}

fn sanitize(value: &str) -> &'static str {
    leak(
        value
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character
                } else {
                    '_'
                }
            })
            .collect(),
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
        // Level B — native exceptions.
        build(
            "exceptions",
            "try-catch-value",
            "(ns b.exceptions.try-catch)\n\
             (defn -main []\n\
               (println (try (throw :boom) (catch Exception error error)))\n\
               (println (try 42 (catch Exception error :unexpected))))\n\
             (-main)\n",
            ":boom\n42\n",
        ),
        build(
            "exceptions",
            "finally-result",
            "(ns b.exceptions.finally-result)\n\
             (defn -main []\n\
               (println (try 42 (finally (println :finally-normal))))\n\
               (println (try (throw 7)\n\
                             (catch Exception error (+ error 1))\n\
                             (finally (println :finally-caught)))))\n\
             (-main)\n",
            ":finally-normal\n42\n:finally-caught\n8\n",
        ),
        build_gc(
            "exceptions",
            "nested-unwind-gc-stress",
            "(ns b.exceptions.nested-unwind)\n\
             (defn -main []\n\
               (let [prefix \"caught:\"]\n\
                 (println\n\
                   (try\n\
                     (try (throw (str prefix \"value\"))\n\
                          (finally (println :inner-finally)))\n\
                     (catch Exception error (str error \"!\"))\n\
                     (finally (println :outer-finally))))))\n\
             (-main)\n",
            ":inner-finally\n:outer-finally\ncaught:value!\n",
        ),
        build_xfail_reason(
            "exceptions",
            "typed-multiple-catches",
            "(ns b.exceptions.typed-catches)\n\
             (defn -main []\n\
               (println\n\
                 (try (throw :boom)\n\
                      (catch ArithmeticException error :arithmetic)\n\
                      (catch Exception error :generic))))\n\
             (-main)\n",
            ":generic\n",
            "The native subset currently accepts one catch-all clause and does not dispatch catches by exception class.",
            "specs/RUNTIME_SPEC.md#erros",
        ),
        build_xfail_reason(
            "exceptions",
            "runtime-fault-catch",
            "(ns b.exceptions.runtime-fault)\n\
             (defn -main []\n\
               (println (try (quot 1 0) (catch Exception error :caught))))\n\
             (-main)\n",
            ":caught\n",
            "Fatal runtime faults such as division by zero are not yet converted into catchable native exception values.",
            "specs/RUNTIME_SPEC.md#erros",
        ),
        // Level B — multimethod dispatch by value.
        build(
            "multimethods",
            "keyword-and-default-dispatch",
            "(ns b.multimethods.keyword-default)\n\
             (defmulti describe (fn [value] (get value :kind)))\n\
             (defmethod describe :circle [value] (str \"circle:\" (get value :radius)))\n\
             (defmethod describe :square [value] (str \"square:\" (get value :side)))\n\
             (defmethod describe :default [value] (str \"unknown:\" (get value :kind)))\n\
             (defn -main []\n\
               (println (describe {:kind :circle :radius 3}))\n\
               (println (describe {:kind :square :side 4}))\n\
               (println (describe {:kind :triangle})))\n\
             (-main)\n",
            "circle:3\nsquare:4\nunknown::triangle\n",
        ),
        build(
            "multimethods",
            "numeric-and-multi-argument-dispatch",
            "(ns b.multimethods.numeric-multi-argument)\n\
             (defmulti parity (fn [number] (mod number 2)))\n\
             (defmethod parity 0 [number] :even)\n\
             (defmethod parity 1 [number] :odd)\n\
             (defmulti calculate (fn [request operand] (get request :operation)))\n\
             (defmethod calculate :add [request operand] (+ (get request :value) operand))\n\
             (defmethod calculate :multiply [request operand] (* (get request :value) operand))\n\
             (defn -main []\n\
               (println (parity 10) (parity 7))\n\
               (println (calculate {:operation :add :value 10} 5)\n\
                        (calculate {:operation :multiply :value 10} 5)))\n\
             (-main)\n",
            ":even :odd\n15 50\n",
        ),
        build_gc(
            "multimethods",
            "structural-dispatch-gc-stress",
            "(ns b.multimethods.structural-dispatch)\n\
             (defmulti route (fn [request] [(get request :kind) (get request :version)]))\n\
             (defmethod route [:invoice 1] [request] :invoice-v1)\n\
             (defmethod route [:invoice 2] [request] :invoice-v2)\n\
             (defmethod route :default [request] :fallback)\n\
             (defn -main []\n\
               (println (route {:kind :invoice :version 1})\n\
                        (route {:kind :invoice :version 2})\n\
                        (route {:kind :other :version 9})))\n\
             (-main)\n",
            ":invoice-v1 :invoice-v2 :fallback\n",
        ),
        build_xfail_reason(
            "multimethods",
            "keyword-dispatch-function",
            "(ns b.multimethods.keyword-dispatch)\n\
             (defmulti describe :kind)\n\
             (defmethod describe :value [item] :matched)\n\
             (defn -main [] (println (describe {:kind :value})))\n\
             (-main)\n",
            ":matched\n",
            "The current analyzer requires an explicit fn as the multimethod dispatch function; invokable keywords are not supported.",
            "specs/LANGUAGE_SCOPE.md#abstrações",
        ),
        build_xfail_reason(
            "multimethods",
            "hierarchy-dispatch",
            "(ns b.multimethods.hierarchy)\n\
             (derive :dog :animal)\n\
             (defmulti speak (fn [value] value))\n\
             (defmethod speak :animal [value] :generic-animal)\n\
             (defn -main [] (println (speak :dog)))\n\
             (-main)\n",
            ":generic-animal\n",
            "Multimethod hierarchy dispatch through derive and isa? is not implemented; only equality and :default fallback are available.",
            "specs/LANGUAGE_SCOPE.md#abstrações",
        ),
        // Level B — transient collection construction.
        build(
            "collections/transients",
            "vector-operations",
            "(ns b.transients.vector)\n\
             (defn -main []\n\
               (let [value (transient [10 20 30])\n\
                     value (assoc! value 1 99)\n\
                     value (conj! value 40)]\n\
                 (println (count value) (nth value 1) (get value 3))\n\
                 (println (persistent! value)))\n\
               (println (persistent! (transient []))))\n\
             (-main)\n",
            "4 99 40\n[10 99 30 40]\n[]\n",
        ),
        build(
            "collections/transients",
            "map-and-set-operations",
            "(ns b.transients.map-set)\n\
             (defn -main []\n\
               (let [value (transient {:a 1 :b 2})\n\
                     value (assoc! value :c 3)\n\
                     value (dissoc! value :b)]\n\
                 (println (count value) (get value :c) (contains? value :b))\n\
                 (println (persistent! value)))\n\
               (let [value (conj! (conj! (transient #{1}) 2) 3)]\n\
                 (println (count value) (contains? value 2) (persistent! value))))\n\
             (-main)\n",
            "2 3 false\n{:a 1, :c 3}\n3 true #{1 2 3}\n",
        ),
        build_gc(
            "collections/transients",
            "bulk-build-gc-stress",
            "(ns b.transients.bulk)\n\
             (defn build-vector [limit]\n\
               (loop [index 0 value (transient [])]\n\
                 (if (< index limit)\n\
                   (recur (inc index) (conj! value (* index index)))\n\
                   (persistent! value))))\n\
             (defn build-map [limit]\n\
               (loop [index 0 value (transient {})]\n\
                 (if (< index limit)\n\
                   (recur (inc index) (assoc! value index (inc index)))\n\
                   (persistent! value))))\n\
             (defn -main []\n\
               (let [vector-value (build-vector 128)\n\
                     map-value (build-map 96)]\n\
                 (println (count vector-value) (nth vector-value 127))\n\
                 (println (count map-value) (get map-value 95))))\n\
             (-main)\n",
            "128 16129\n96 96\n",
        ),
        build_xfail_reason(
            "collections/transients",
            "use-after-persistent",
            "(ns b.transients.use-after-persistent)\n\
             (defn -main []\n\
               (println\n\
                 (try\n\
                   (let [value (transient [])]\n\
                     (persistent! value)\n\
                     (conj! value 1)\n\
                     :unexpected)\n\
                   (catch Exception error :caught))))\n\
             (-main)\n",
            ":caught\n",
            "Transient edit ownership is not invalidated after persistent!, so use-after-persistent does not yet raise the Clojure error.",
            "specs/RUNTIME_SPEC.md#coleções-persistentes",
        ),
        build_xfail_reason(
            "collections/transients",
            "disj-bang",
            "(ns b.transients.disj)\n\
             (defn -main []\n\
               (println (persistent! (disj! (transient #{1 2}) 1))))\n\
             (-main)\n",
            "#{2}\n",
            "The initial transient subset implements conj!, assoc!, and dissoc!; disj! and pop! remain unavailable.",
            "specs/RUNTIME_SPEC.md#coleções-persistentes",
        ),
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

    // Level B — the currently executable stdout baseline. These cases keep
    // `print`/`println` separate from the future stream-redirection contract.
    cases.extend([
        build(
            "io/output",
            "print-normal",
            "(ns b.io.print-normal)\n(defn -main [] (print \"hello\" 42))\n(-main)\n",
            "hello 42",
        ),
        build(
            "io/output",
            "print-empty",
            "(ns b.io.print-empty)\n(defn -main [] (print \"\") (println :done))\n(-main)\n",
            ":done\n",
        ),
        build(
            "io/output",
            "print-unicode",
            "(ns b.io.print-unicode)\n(defn -main [] (print \"ação λ 東京 😀\"))\n(-main)\n",
            "ação λ 東京 😀",
        ),
        build(
            "io/output",
            "println-normal",
            "(ns b.io.println-normal)\n(defn -main [] (println \"hello\" 42))\n(-main)\n",
            "hello 42\n",
        ),
        build(
            "io/output",
            "println-empty",
            "(ns b.io.println-empty)\n(defn -main [] (println))\n(-main)\n",
            "\n",
        ),
        build(
            "io/output",
            "println-unicode",
            "(ns b.io.println-unicode)\n(defn -main [] (println \"ação λ 東京 😀\"))\n(-main)\n",
            "ação λ 東京 😀\n",
        ),
    ]);

    // Level B — language/runtime prerequisites that make the higher-level I/O
    // APIs safe under redirection, exceptions, GC, and abnormal exit.
    cases.extend([
        build_xfail(
            "io/dynamic-bindings",
            "out-restored",
            "(ns b.io.out-restored)\n(defn -main []\n  (let [capture (cljn.io/string-writer)]\n    (binding [*out* capture] (print \"captured\"))\n    (println \"terminal\")))\n(-main)\n",
            "terminal\n",
            "specs/IO_SPEC.md#standard-streams-and-process-context",
        ),
        with_expected_stderr(
            build_xfail(
                "io/dynamic-bindings",
                "stderr-redirection",
                "(ns b.io.stderr)\n(defn -main []\n  (binding [*out* *err*] (println \"problem\"))\n  (println \"ok\"))\n(-main)\n",
                "ok\n",
                "specs/IO_SPEC.md#standard-streams-and-process-context",
            ),
            "problem\n",
        ),
        build_xfail(
            "io/dynamic-bindings",
            "flush-on-newline-disabled",
            "(ns b.io.flush-binding)\n(defn -main []\n  (binding [*flush-on-newline* false] (println \"buffered\") (flush)))\n(-main)\n",
            "buffered\n",
            "specs/IO_SPEC.md#standard-streams-and-process-context",
        ),
        build_xfail(
            "io/exceptions",
            "catch-io-exception-data",
            "(ns b.io.catch)\n(defn -main []\n  (try (slurp \"missing.txt\")\n       (catch cljn.io/IOException error\n         (println (:kind (ex-data error)) (:operation (ex-data error))))))\n(-main)\n",
            ":not-found :open-reader\n",
            "specs/IO_SPEC.md#erros",
        ),
        build_xfail(
            "io/exceptions",
            "finally-runs",
            "(ns b.io.finally)\n(defn -main []\n  (try (throw (ex-info \"boom\" {}))\n       (catch Exception error (print \"caught \"))\n       (finally (println \"closed\"))))\n(-main)\n",
            "caught closed\n",
            "specs/IO_SPEC.md#dependências-bloqueantes",
        ),
        build_xfail(
            "io/exceptions",
            "binding-restored-on-unwind",
            "(ns b.io.unwind-binding)\n(defn -main []\n  (try (binding [*out* (cljn.io/string-writer)] (throw (ex-info \"boom\" {})))\n       (catch Exception error (println \"restored\"))))\n(-main)\n",
            "restored\n",
            "specs/IO_SPEC.md#dependências-bloqueantes",
        ),
        native_build_xfail(
            "io/lifecycle",
            "close-idempotent",
            "(ns b.io.close-idempotent)\n(defn -main []\n  (let [r (cljn.io/string-reader \"x\")]\n    (cljn.io/close! r)\n    (cljn.io/close! r)\n    (println (cljn.io/closed? r))))\n(-main)\n",
            "true\n",
        ),
        native_build_xfail(
            "io/lifecycle",
            "use-after-close",
            "(ns b.io.use-after-close)\n(defn -main []\n  (let [r (cljn.io/string-reader \"x\")]\n    (cljn.io/close! r)\n    (try (cljn.io/read-char r)\n         (catch cljn.io/IOException error (println (:kind (ex-data error)))))))\n(-main)\n",
            ":closed\n",
        ),
        native_build_xfail(
            "io/lifecycle",
            "with-open-exception",
            "(ns b.io.with-open-exception)\n(defn -main []\n  (let [r (cljn.io/string-reader \"x\")]\n    (try (with-open [opened r] (throw (ex-info \"boom\" {})))\n         (catch Exception error (println (cljn.io/closed? r))))))\n(-main)\n",
            "true\n",
        ),
        native_build_xfail(
            "io/binary",
            "roundtrip-zero-and-ff",
            "(ns b.io.binary-roundtrip)\n(defn -main []\n  (let [out (cljn.io/byte-output-stream)]\n    (cljn.io/write-bytes! out (cljn.io/bytes [0 255]))\n    (println (cljn.io/bytes->vector (cljn.io/output-bytes out)))))\n(-main)\n",
            "[0 255]\n",
        ),
        native_build_xfail(
            "io/files",
            "large-file-streaming",
            "(ns b.io.large-file)\n(defn -main []\n  (with-open [in (cljn.io/input-stream \"large.bin\")\n              out (cljn.io/output-stream \"copy.bin\")]\n    (loop [total 0]\n      (if-let [chunk (cljn.io/read-bytes in 65536)]\n        (do (cljn.io/write-bytes! out chunk)\n            (recur (+ total (cljn.io/byte-count chunk))))\n        (println total)))))\n(-main)\n",
            "1048576\n",
        ),
    ]);
    cases
        .iter_mut()
        .find(|case| case.id == "b.io_files.large_file_streaming")
        .expect("large-file I/O case")
        .extra_binary_files = Box::leak(
        vec![(
            "work.before/large.bin",
            &*Box::leak(vec![0x5a_u8; 1_048_576].into_boxed_slice()),
        )]
        .into_boxed_slice(),
    );

    // Levels B/C — complete executable inventory for the proposed native I/O
    // surface. All missing operations are xfail rather than undocumented
    // aspirations, and every function has normal, boundary, and error cases.
    cases.extend(io_api_cases());

    let stdin_case = with_run(
        with_binary_files(
            build_xfail(
                "io/input",
                "stdin-crlf-eof",
                "(ns b.io.stdin)\n(defn -main [] (println (read-line) (read-line) (read-line)))\n(-main)\n",
                "alpha beta nil\n",
                "specs/IO_SPEC.md#standard-streams-and-process-context",
            ),
            &[("stdin-crlf.bin", b"alpha\r\nbeta\n")],
        ),
        "[run]\nstdin = \"stdin-crlf.bin\"\nplatforms = [\"linux\"]\n",
    );
    cases.push(stdin_case);

    let invalid_utf8_case = with_run(
        with_binary_files(
            fixture(
                'B',
                "io/input",
                "stdin-invalid-utf8",
                "b.io.input.stdin_invalid_utf8",
                "semantics/io/input",
                "xfail",
                "unsupported",
                "build-run",
                "not-applicable",
                false,
                "Strict UTF-8 input must raise a categorized native I/O exception.",
                "specs/IO_SPEC.md#encoding-and-line-rules",
                None,
                "(ns b.io.invalid-utf8)\n(defn -main []\n  (println\n    (try\n      (do (read-line) :unexpected)\n      (catch E error (get error :kind)))))\n(-main)\n",
                Expected::Stdout(":invalid-input\n"),
            ),
            INVALID_UTF8_FILE,
        ),
        "[run]\nstdin = \"stdin-invalid.bin\"\nplatforms = [\"linux\"]\n",
    );
    cases.push(invalid_utf8_case);

    let mut process_context_case = build_xfail(
            "io/process",
            "argv-environment-cwd",
            "(ns b.io.process)\n(defn -main []\n  (println *command-line-args*)\n  (println (cljn.process/getenv \"CLJN_CONFORMANCE_VALUE\"))\n  (println (cljn.io/absolute? (cljn.process/cwd))))\n(-main)\n",
            "(\"first\" \"ação\")\nvisible\ntrue\n",
            "specs/IO_SPEC.md#standard-streams-and-process-context",
        );
    process_context_case.oracle = "not-applicable";
    cases.push(with_run(
        process_context_case,
        "[run]\nargs = [\"first\", \"ação\"]\nenv = { CLJN_CONFORMANCE_VALUE = \"visible\" }\nplatforms = [\"linux\"]\n",
    ));

    cases.push(with_run(
        with_extra_files(
            fixture(
                'C',
                "cljn-io/filesystem/contracts",
                "isolated-tree-and-symlink",
                "c.cljn_io.filesystem.isolated_tree_and_symlink",
                "stdlib/cljn-io/filesystem",
                "xfail",
                "unsupported",
                "build-run",
                "not-applicable",
                true,
                "Recursive operations and symlink policy require an exact isolated filesystem oracle.",
                "specs/IO_SPEC.md#filesystem-and-recursive-safety",
                Some("cljn.io"),
                "(ns c.io.tree)\n(defn -main []\n  (cljn.io/copy-tree! \"source\" \"copy\")\n  (cljn.io/delete-tree! \"source\")\n  (println :ok))\n(-main)\n",
                Expected::Stdout(":ok\n"),
            ),
            &[
                ("work.before/source/a.txt", "a\n"),
                ("work.after/copy/a.txt", "a\n"),
            ],
        ),
        "[run]\nplatforms = [\"linux\"]\nsetup_symlinks = [{ path = \"source/link\", target = \"a.txt\" }]\nexpected_symlinks = [{ path = \"copy/link\", target = \"a.txt\" }]\n",
    ));

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

    // Level D — executable pure-library slices.
    cases.extend([
        higher_level_build(
            'D',
            "functional",
            "collection-pipeline",
            "pure-libraries/functional",
            "(ns d.functional-pipeline)\n\
             (defn transform [xs]\n\
               (mapv (fn [x] (+ (* x x) 1)) (filter odd? xs)))\n\
             (defn -main [] (println (transform (range 8))))\n\
             (-main)\n",
            "[2 10 26 50]\n",
            false,
        ),
        higher_level_build(
            'D',
            "functional",
            "closure-api",
            "pure-libraries/functional",
            "(ns d.closure-api)\n\
             (defn make-affine [a b] (fn [x] (+ (* a x) b)))\n\
             (defn apply-all [f xs] (mapv f xs))\n\
             (defn -main [] (println (apply-all (make-affine 3 2) [1 2 3 4])))\n\
             (-main)\n",
            "[5 8 11 14]\n",
            false,
        ),
        higher_level_build(
            'D',
            "functional",
            "variadic-api",
            "pure-libraries/functional",
            "(ns d.variadic-api)\n\
             (defn total [x & xs] (reduce + x xs))\n\
             (defn -main []\n\
               (println (total 1) (total 1 2 3 4) (apply total [10 20 12])))\n\
             (-main)\n",
            "1 10 42\n",
            false,
        ),
        higher_level_build(
            'D',
            "persistent-structures",
            "tree-library",
            "pure-libraries/persistent-structures",
            "(ns d.tree-library)\n\
             (defn node [value left right] {:value value :left left :right right})\n\
             (defn total [tree]\n\
               (if (nil? tree)\n\
                 0\n\
                 (+ (:value tree) (total (:left tree)) (total (:right tree)))))\n\
             (defn depth [tree]\n\
               (if (nil? tree)\n\
                 0\n\
                 (inc (max (depth (:left tree)) (depth (:right tree))))))\n\
             (defn -main []\n\
               (let [tree (node 5 (node 3 (node 1 nil nil) nil) (node 6 nil nil))]\n\
                 (println (total tree) (depth tree))))\n\
             (-main)\n",
            "15 3\n",
            false,
        ),
        higher_level_build(
            'D',
            "persistent-structures",
            "index-library",
            "pure-libraries/persistent-structures",
            "(ns d.index-library)\n\
             (defn build-index [pairs]\n\
               (reduce (fn [index pair]\n\
                         (assoc index (first pair) (second pair)))\n\
                       {}\n\
                       pairs))\n\
             (defn -main []\n\
               (let [index (build-index [[:a 10] [:b 20] [:c 30]])]\n\
                 (println (count index) (get index :b) (contains? index :c)\n\
                          (count (dissoc index :a)))))\n\
             (-main)\n",
            "3 20 true 2\n",
            false,
        ),
        higher_level_build(
            'D',
            "protocols-records",
            "domain-model",
            "pure-libraries/protocols-records",
            "(ns d.domain-model)\n\
             (defprotocol Costed (cost [item]))\n\
             (defrecord LineItem [price quantity])\n\
             (extend-type LineItem Costed\n\
               (cost [item] (* (:price item) (:quantity item))))\n\
             (defn total-cost [items]\n\
               (reduce (fn [sum item] (+ sum (cost item))) 0 items))\n\
             (defn -main []\n\
               (let [items [(->LineItem 10 2) (->LineItem 7 3) (->LineItem 99 0)]]\n\
                 (println (mapv cost items) (total-cost items))))\n\
             (-main)\n",
            "[20 21 0] 41\n",
            false,
        ),
        higher_level_build(
            'D',
            "protocols-records",
            "builtin-extension",
            "pure-libraries/protocols-records",
            "(ns d.builtin-extension)\n\
             (defprotocol Summarized (summary [value]))\n\
             (extend-type List Summarized\n\
               (summary [value] (reduce + 0 value)))\n\
             (defn -main []\n\
               (println (summary (list)) (summary (list 1 2 3 4))))\n\
             (-main)\n",
            "0 10\n",
            false,
        ),
        higher_level_build(
            'D',
            "persistent-structures",
            "gc-stress-index",
            "pure-libraries/persistent-structures",
            "(ns d.gc-stress-index)\n\
             (defn build-index [n]\n\
               (loop [i 0 index {}]\n\
                 (if (< i n)\n\
                   (recur (inc i) (assoc index i [i (* i i)]))\n\
                   index)))\n\
             (defn -main []\n\
               (let [index (build-index 80)]\n\
                 (println (count index) (get index 79) (contains? index 40))))\n\
             (-main)\n",
            "80 [79 6241] true\n",
            true,
        ),
    ]);

    // Level D — executable gaps plus exception recovery at a library boundary.
    cases.extend([
        higher_level_xfail(
            'D',
            "macros",
            "user-macro-api",
            "pure-libraries/macros",
            "(ns d.user-macro)\n\
             (defmacro unless [predicate value] `(if ~predicate nil ~value))\n\
             (defn -main [] (println (unless false 42)))\n\
             (-main)\n",
            "42\n",
            "equal",
            "Pure libraries that define macros still require user macro expansion.",
        ),
        higher_level_xfail(
            'D',
            "namespaces",
            "cross-namespace-api",
            "pure-libraries/namespaces",
            "(ns d.consumer (:require [d.math-library :as math]))\n\
             (defn -main [] (println (math/answer)))\n\
             (-main)\n",
            "42\n",
            "not-applicable",
            "A library consumer cannot yet load another source namespace.",
        ),
        higher_level_xfail(
            'D',
            "sequences",
            "lazy-pipeline",
            "pure-libraries/sequences",
            "(ns d.lazy-pipeline)\n\
             (defn -main [] (println (take 5 (iterate inc 0))))\n\
             (-main)\n",
            "(0 1 2 3 4)\n",
            "equal",
            "Pure libraries using lazy or infinite sequences are not yet executable.",
        ),
        higher_level_xfail(
            'D',
            "metadata",
            "metadata-api",
            "pure-libraries/metadata",
            "(ns d.metadata-api)\n\
             (defn -main []\n\
               (println (:role (meta (with-meta [] {:role :data})))))\n\
             (-main)\n",
            ":data\n",
            "equal",
            "Runtime metadata functions are not available on compiled values.",
        ),
        higher_level_xfail(
            'D',
            "errors",
            "typed-exception-api",
            "pure-libraries/errors",
            "(ns d.typed-exception-api)\n\
             (defn classify-failure [value]\n\
               (try (throw value)\n\
                    (catch ArithmeticException error :arithmetic)\n\
                    (catch Exception error :generic)))\n\
             (defn -main [] (println (classify-failure :boom)))\n\
             (-main)\n",
            ":generic\n",
            "equal",
            "Reusable error APIs still cannot select among multiple catches by native exception type.",
        ),
        higher_level_build(
            'D',
            "errors",
            "exception-api",
            "pure-libraries/errors",
            "(ns d.exception-api)\n\
             (defn require-positive [value]\n\
               (if (pos? value) value (throw :not-positive)))\n\
             (defn -main []\n\
               (println (require-positive 7))\n\
               (println (try (require-positive 0)\n\
                             (catch Exception error [:caught error])\n\
                             (finally (println :validated)))))\n\
             (-main)\n",
            "7\n:validated\n[:caught :not-positive]\n",
            true,
        ),
    ]);

    // Level D — project-shaped inventory that still requires a loader.
    cases.extend([
        pending_project(
            'D',
            "functional-library",
            "pure-libraries/functional",
            "Requires a multi-file project loader for a pure functional library.",
            "(ns fixture.functional)\n(defn transform [xs] (map inc xs))\n",
        ),
        pending_project(
            'D',
            "macro-library",
            "pure-libraries/macros",
            "Requires build-time loading of macros from another namespace.",
            "(ns fixture.macros)\n(defmacro unless [p x] `(if ~p nil ~x))\n",
        ),
        pending_project(
            'D',
            "multiple-namespaces",
            "pure-libraries/namespaces",
            "Requires dependency ordering across multiple source files.",
            "(ns fixture.app (:require [fixture.lib :as lib]))\n(lib/value)\n",
        ),
        pending_project(
            'D',
            "protocol-record-library",
            "pure-libraries/protocols-records",
            "Requires project-level protocol and record definitions.",
            "(ns fixture.protocols)\n(defprotocol P (value [x]))\n",
        ),
        pending_project(
            'D',
            "persistent-structure-library",
            "pure-libraries/persistent-structures",
            "Requires a reusable project that builds a persistent data structure.",
            "(ns fixture.tree)\n(defn node [v l r] {:value v :left l :right r})\n",
        ),
    ]);

    // Level E — self-contained native application baseline.
    cases.extend([
        higher_level_build(
            'E',
            "applications",
            "invoice-cli",
            "ecosystem/application",
            "(ns e.invoice-cli)\n\
             (defrecord Item [price quantity])\n\
             (defn line-total [item] (* (:price item) (:quantity item)))\n\
             (defn invoice-total [items]\n\
               (reduce (fn [sum item] (+ sum (line-total item))) 0 items))\n\
             (defn -main []\n\
               (let [items [(->Item 12 2) (->Item 5 3) (->Item 9 1)]]\n\
                 (println \"invoice\" (count items) (invoice-total items))))\n\
             (-main)\n",
            "invoice 3 48\n",
            false,
        ),
        higher_level_build(
            'E',
            "applications",
            "analytics-cli",
            "ecosystem/application",
            "(ns e.analytics-cli)\n\
             (defn positives [values] (filter pos? values))\n\
             (defn -main []\n\
               (let [values [-2 0 3 4 -1 5]]\n\
                 (println \"report\" (count values) (reduce + 0 values)\n\
                          (mapv (fn [x] (* x x)) (positives values)))))\n\
             (-main)\n",
            "report 6 9 [9 16 25]\n",
            false,
        ),
        higher_level_build(
            'E',
            "applications",
            "polymorphic-service",
            "ecosystem/application",
            "(ns e.polymorphic-service)\n\
             (defprotocol Handler (handle [request]))\n\
             (defrecord AddRequest [left right])\n\
             (defrecord MultiplyRequest [left right])\n\
             (extend-type AddRequest Handler\n\
               (handle [request] (+ (:left request) (:right request))))\n\
             (extend-type MultiplyRequest Handler\n\
               (handle [request] (* (:left request) (:right request))))\n\
             (defn -main []\n\
               (println (mapv handle [(->AddRequest 20 22)\n\
                                      (->MultiplyRequest 6 7)])))\n\
             (-main)\n",
            "[42 42]\n",
            false,
        ),
        higher_level_build(
            'E',
            "applications",
            "graph-report",
            "ecosystem/application",
            "(ns e.graph-report)\n\
             (defn reachable-count [graph start]\n\
               (loop [queue (list start) seen #{}]\n\
                 (if (empty? queue)\n\
                   (count seen)\n\
                   (let [node (first queue) remaining (rest queue)]\n\
                     (if (contains? seen node)\n\
                       (recur remaining seen)\n\
                       (recur (concat remaining (get graph node))\n\
                              (conj seen node)))))))\n\
             (defn -main []\n\
               (println \"reachable\"\n\
                        (reachable-count {0 [1 2] 1 [3] 2 [3 4] 3 [] 4 []} 0)))\n\
             (-main)\n",
            "reachable 5\n",
            false,
        ),
        higher_level_build(
            'E',
            "applications",
            "gc-stress-workload",
            "ecosystem/application",
            "(ns e.gc-stress-workload)\n\
             (defn squares [n] (mapv (fn [x] (* x x)) (range n)))\n\
             (defn -main []\n\
               (let [values (squares 200)]\n\
                 (println (count values) (reduce + 0 values))))\n\
             (-main)\n",
            "200 2646700\n",
            true,
        ),
    ]);

    // Level E — concrete ecosystem gaps.
    cases.extend([
        higher_level_xfail(
            'E',
            "dependencies",
            "external-library",
            "ecosystem/dependencies",
            "(ns e.external-library (:require [cheshire.core :as json]))\n\
             (defn -main [] (println (json/generate-string {:ok true})))\n\
             (-main)\n",
            "{\"ok\":true}\n",
            "not-applicable",
            "External source dependencies cannot be resolved by the current build command.",
        ),
        higher_level_xfail(
            'E',
            "jar-classpath",
            "foreign-class",
            "ecosystem/jar-classpath",
            "(ns e.foreign-class)\n\
             (defn -main []\n\
               (println (org.apache.commons.lang3.StringUtils/upperCase \"clojure\")))\n\
             (-main)\n",
            "CLOJURE\n",
            "not-applicable",
            "JAR classpath lookup is outside the native runtime.",
        ),
        higher_level_xfail(
            'E',
            "java-interop",
            "static-method",
            "ecosystem/java-interop",
            "(ns e.java-static)\n\
             (defn -main [] (println (Math/abs -7)))\n\
             (-main)\n",
            "7\n",
            "equal",
            "Java static method interop has no native execution path.",
        ),
        higher_level_xfail(
            'E',
            "dynamic-loading",
            "runtime-require",
            "ecosystem/dynamic-loading",
            "(ns e.runtime-require)\n\
             (defn -main []\n\
               (require 'clojure.set)\n\
               (println (clojure.set/union #{1 2} #{2 3})))\n\
             (-main)\n",
            "#{1 2 3}\n",
            "not-applicable",
            "Runtime require and namespace loading are not implemented.",
        ),
        higher_level_xfail(
            'E',
            "dynamic-loading",
            "runtime-eval",
            "ecosystem/dynamic-loading",
            "(ns e.runtime-eval)\n\
             (defn -main [] (println (eval '(+ 20 22))))\n\
             (-main)\n",
            "42\n",
            "equal",
            "Runtime eval is not part of the AOT executable.",
        ),
        higher_level_xfail(
            'E',
            "concurrency",
            "future-api",
            "ecosystem/concurrency",
            "(ns e.future-api)\n\
             (defn -main [] (println @(future (+ 20 22))))\n\
             (-main)\n",
            "42\n",
            "equal",
            "Threads and futures are outside the current single-threaded runtime.",
        ),
    ]);

    // Level E — project/ecosystem inventory without an execution path yet.
    cases.extend([
        pending_project('E', "external-dependency", "ecosystem/dependencies", "No dependency resolver exists in the native project path.", "(ns fixture.external (:require [external.lib :as ext]))\n(ext/run)\n"),
        pending_project('E', "jar-classpath", "ecosystem/jar-classpath", "JAR and JVM classpath loading are outside the native runtime.", "(ns fixture.jar)\n"),
        pending_project('E', "java-interop", "ecosystem/java-interop", "Java interop has no native equivalent.", "(ns fixture.java)\n(System/currentTimeMillis)\n"),
        pending_project('E', "dynamic-loading", "ecosystem/dynamic-loading", "Dynamic require and eval are not part of the current AOT path.", "(ns fixture.dynamic)\n(require 'fixture.other)\n"),
        pending_project('E', "integrated-application", "ecosystem/application", "An integrated application needs modules, resources, dependencies, and stable packaging.", "(ns fixture.application)\n(defn -main [] (println \"app\"))\n"),
        pedestal_hello_world_project(),
    ]);

    cases
}
