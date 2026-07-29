//! `clojure-conformance` command-line interface.
//!
//! The executable exposes deterministic fixture verification and listing as
//! normal offline operations. JVM comparison and blessing are explicit oracle
//! commands that require the pinned Clojure classpath. All commands delegate
//! schema, isolation, comparison, checksum, and report behavior to
//! `clojure_test_support`.

use clojure_test_support::{
    human_summary, list_cases, load_reader_coverage, parse_level, parse_status,
    reader_coverage_summary, run_oracle, verify, Filters, OracleMode, OracleOptions, VerifyOptions,
    MAX_JOBS,
};
use std::path::PathBuf;
use std::process::ExitCode;
use std::{env, fs};

struct Common {
    root: PathBuf,
    report_directory: PathBuf,
    filters: Filters,
    compiler: PathBuf,
    jobs: usize,
    classpath: Option<String>,
    java: PathBuf,
    ir_optimization: Option<String>,
    ir_experiment: Option<String>,
}

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(success) if success => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("conformance: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<bool, String> {
    let Some(command) = args.first().map(String::as_str) else {
        print_usage();
        return Ok(false);
    };
    if matches!(command, "help" | "-h" | "--help") {
        print_usage();
        return Ok(true);
    }
    match command {
        "verify" => {
            let common = parse_common(&args[1..])?;
            let report = verify(&VerifyOptions {
                root: common.root,
                compiler: common.compiler,
                report_directory: common.report_directory,
                jobs: common.jobs,
                filters: common.filters,
                ir_optimization: common.ir_optimization,
                ir_experiment: common.ir_experiment,
            })?;
            println!("{}", human_summary(&report));
            Ok(report.success)
        }
        "list" => {
            let common = parse_common(&args[1..])?;
            let cases = list_cases(&common.root, &common.filters)?;
            println!("STATUS   LEVEL AREA                     ID");
            for case in &cases {
                println!(
                    "{:<8} {:<5} {:<24} {}",
                    format!("{:?}", case.manifest.status).to_ascii_lowercase(),
                    format!("{:?}", case.manifest.level),
                    case.manifest.area,
                    case.manifest.id
                );
            }
            println!("{} case(s)", cases.len());
            Ok(true)
        }
        "oracle" => {
            let Some(mode) = args.get(1) else {
                return Err("oracle requires --check or --bless".to_string());
            };
            let mode = match mode.as_str() {
                "--check" => OracleMode::Check,
                "--bless" => OracleMode::Bless,
                _ => return Err("oracle requires --check or --bless".to_string()),
            };
            let common = parse_common(&args[2..])?;
            let classpath = common
                .classpath
                .or_else(|| env::var("CLOJURE_CLASSPATH").ok())
                .ok_or_else(|| {
                    "set CLOJURE_CLASSPATH to the Clojure/JVM 1.12.5 jars or pass --classpath"
                        .to_string()
                })?;
            let helper = common.root.join("oracle/runner.clj");
            let report = run_oracle(&OracleOptions {
                mode,
                root: common.root,
                report_directory: common.report_directory,
                classpath,
                java: common.java,
                helper,
                filters: common.filters,
            })?;
            println!("{}", human_summary(&report));
            Ok(report.success)
        }
        "reader-coverage" => {
            let options = parse_reader_coverage(&args[1..])?;
            let report = load_reader_coverage(&options.catalog, &options.root)?;
            let summary = reader_coverage_summary(&report);
            fs::create_dir_all(&options.report_directory).map_err(|error| {
                format!(
                    "cannot create {}: {error}",
                    options.report_directory.display()
                )
            })?;
            let json = serde_json::to_string_pretty(&report)
                .map_err(|error| format!("cannot serialize reader coverage: {error}"))?;
            fs::write(
                options.report_directory.join("reader-syntax-coverage.json"),
                format!("{json}\n"),
            )
            .map_err(|error| format!("cannot write reader coverage JSON: {error}"))?;
            fs::write(
                options.report_directory.join("reader-syntax-coverage.txt"),
                format!("{summary}\n"),
            )
            .map_err(|error| format!("cannot write reader coverage summary: {error}"))?;
            if options.json {
                println!("{json}");
            } else {
                println!("{summary}");
            }
            Ok(true)
        }
        other => Err(format!("unknown command `{other}`")),
    }
}

struct ReaderCoverageOptions {
    root: PathBuf,
    catalog: PathBuf,
    report_directory: PathBuf,
    json: bool,
}

fn parse_reader_coverage(args: &[String]) -> Result<ReaderCoverageOptions, String> {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut options = ReaderCoverageOptions {
        root: repository.join("tests/conformance"),
        catalog: repository.join("specs/conformance/clojure-1.12.5-reader.toml"),
        report_directory: repository.join("target/conformance"),
        json: false,
    };
    let mut index = 0;
    while index < args.len() {
        if args[index] == "--json" {
            options.json = true;
            index += 1;
            continue;
        }
        let flag = &args[index];
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("{flag} requires a value"))?;
        match flag.as_str() {
            "--root" => options.root = PathBuf::from(value),
            "--catalog" => options.catalog = PathBuf::from(value),
            "--report-directory" => options.report_directory = PathBuf::from(value),
            _ => return Err(format!("unknown reader-coverage option `{flag}`")),
        }
        index += 2;
    }
    Ok(options)
}

fn parse_common(args: &[String]) -> Result<Common, String> {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut common = Common {
        root: repository.join("tests/conformance"),
        report_directory: repository.join("target/conformance"),
        filters: Filters::default(),
        compiler: repository.join("target/release/clojure-native"),
        jobs: MAX_JOBS,
        classpath: None,
        java: PathBuf::from("java"),
        ir_optimization: None,
        ir_experiment: None,
    };
    let mut index = 0;
    while index < args.len() {
        let flag = &args[index];
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("{flag} requires a value"))?;
        match flag.as_str() {
            "--root" => common.root = PathBuf::from(value),
            "--report-directory" => common.report_directory = PathBuf::from(value),
            "--compiler" => common.compiler = PathBuf::from(value),
            "--jobs" => {
                common.jobs = value
                    .parse()
                    .map_err(|_| format!("invalid --jobs value `{value}`"))?;
            }
            "--level" => common.filters.level = Some(parse_level(value)?),
            "--area" => common.filters.area = Some(value.clone()),
            "--status" => common.filters.status = Some(parse_status(value)?),
            "--namespace" => common.filters.namespace = Some(value.clone()),
            "--classpath" => common.classpath = Some(value.clone()),
            "--java" => common.java = PathBuf::from(value),
            "--ir-opt" => match value.as_str() {
                "none" | "safe" => common.ir_optimization = Some(value.clone()),
                _ => {
                    return Err(format!(
                        "invalid --ir-opt value `{value}`; expected none or safe"
                    ))
                }
            },
            "--ir-experiment" => match value.as_str() {
                "none" | "adr15" => common.ir_experiment = Some(value.clone()),
                _ => {
                    return Err(format!(
                        "invalid --ir-experiment value `{value}`; expected none or adr15"
                    ))
                }
            },
            _ => return Err(format!("unknown option `{flag}`")),
        }
        index += 2;
    }
    if common
        .ir_experiment
        .as_deref()
        .is_some_and(|value| value != "none")
        && common.ir_optimization.as_deref() != Some("safe")
    {
        return Err("--ir-experiment adr15 requires --ir-opt safe".to_string());
    }
    Ok(common)
}

fn print_usage() {
    println!(
        "clojure-conformance\n\n\
         Usage:\n\
           clojure-conformance verify [filters] [--jobs 1..4] [--ir-opt none|safe] [--ir-experiment none|adr15]\n\
           clojure-conformance list [filters]\n\
           clojure-conformance reader-coverage [--json] [--catalog PATH]\n\
           clojure-conformance oracle --check [filters] [--classpath PATH]\n\
           clojure-conformance oracle --bless [filters] [--classpath PATH]\n\n\
         Filters:\n\
           --level A|B|C|D|E\n\
           --area TEXT\n\
           --status active|xfail|pending\n\
           --namespace TEXT\n\n\
         Paths:\n\
           --root PATH --compiler PATH --report-directory PATH\n\
           --catalog PATH (reader-coverage only)\n\n\
         The JVM oracle is manual and is pinned to Clojure 1.12.5."
    );
}
