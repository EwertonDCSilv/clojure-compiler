//! Analyze paired native benchmark samples for ADR-0014.
//!
//! The tool intentionally uses only the Rust standard library so the benchmark
//! gate can be reproduced with the repository's existing Rust toolchain.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
struct Sample {
    benchmark: String,
    repetition: usize,
    profile: String,
    wall: f64,
    cpu: f64,
    rss: u64,
    checksum: String,
    status: String,
}

#[derive(Clone, Copy, Debug)]
struct Pair {
    control_wall: f64,
    candidate_wall: f64,
    control_cpu: f64,
    candidate_cpu: f64,
    control_rss: u64,
    candidate_rss: u64,
}

impl Pair {
    fn wall_ratio(self) -> f64 {
        ratio(self.candidate_wall, self.control_wall)
    }

    fn cpu_ratio(self) -> f64 {
        ratio(self.candidate_cpu, self.control_cpu)
    }
}

#[derive(Clone, Copy, Debug)]
struct Estimate {
    median: f64,
    mad: f64,
    low: f64,
    high: f64,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("análise IR A/B: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() < 2 {
        return Err(
            "uso: analyze-ir-ab RAW.csv REPORT.md --repetitions N --scale N [--partial]"
                .to_string(),
        );
    }
    let raw = &arguments[0];
    let report = &arguments[1];
    let mut repetitions = None;
    let mut scale = None;
    let mut partial = false;
    let mut index = 2;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--repetitions" => {
                repetitions = Some(parse_usize(arguments.get(index + 1), "--repetitions")?);
                index += 2;
            }
            "--scale" => {
                scale = Some(parse_usize(arguments.get(index + 1), "--scale")?);
                index += 2;
            }
            "--partial" => {
                partial = true;
                index += 1;
            }
            option => return Err(format!("opção desconhecida: {option}")),
        }
    }
    let repetitions = repetitions.ok_or_else(|| "--repetitions ausente".to_string())?;
    let scale = scale.ok_or_else(|| "--scale ausente".to_string())?;
    let samples = parse_samples(raw)?;
    let pairs = pair_samples(&samples, repetitions)?;
    let markdown = render_report(&pairs, repetitions, scale, partial);
    if let Some(parent) = Path::new(report).parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("não foi possível criar {}: {error}", parent.display()))?;
    }
    fs::write(report, markdown)
        .map_err(|error| format!("não foi possível gravar {report}: {error}"))?;

    let gate_passed = evaluate_gate(&pairs, repetitions, scale, partial);
    if gate_passed {
        println!("Gate IR A/B aprovado; relatório: {report}");
        Ok(())
    } else {
        Err(format!(
            "o perfil ainda não satisfaz o gate da ADR-0014; consulte {report}"
        ))
    }
}

fn parse_usize(value: Option<&String>, option: &str) -> Result<usize, String> {
    value
        .ok_or_else(|| format!("{option} requer um valor"))?
        .parse()
        .map_err(|_| format!("valor inválido para {option}"))
}

fn parse_samples(path: &str) -> Result<Vec<Sample>, String> {
    let input = fs::read_to_string(path)
        .map_err(|error| format!("não foi possível ler {path}: {error}"))?;
    let mut samples = Vec::new();
    for (line_number, line) in input.lines().enumerate().skip(1) {
        let fields = line.split(',').collect::<Vec<_>>();
        if fields.len() != 9 {
            return Err(format!(
                "{path}:{}: esperadas 9 colunas, encontradas {}",
                line_number + 1,
                fields.len()
            ));
        }
        samples.push(Sample {
            benchmark: fields[0].to_string(),
            repetition: fields[1]
                .parse()
                .map_err(|_| format!("{path}:{}: repetição inválida", line_number + 1))?,
            profile: fields[3].to_string(),
            wall: parse_f64(fields[4], path, line_number)?,
            cpu: parse_f64(fields[5], path, line_number)?,
            rss: fields[6]
                .parse()
                .map_err(|_| format!("{path}:{}: RSS inválido", line_number + 1))?,
            checksum: fields[7].to_string(),
            status: fields[8].to_string(),
        });
    }
    if samples.is_empty() {
        return Err("o CSV não contém amostras".to_string());
    }
    Ok(samples)
}

fn parse_f64(value: &str, path: &str, zero_based_line: usize) -> Result<f64, String> {
    let number = value
        .parse::<f64>()
        .map_err(|_| format!("{path}:{}: métrica inválida", zero_based_line + 1))?;
    if number.is_finite() && number >= 0.0 {
        Ok(number)
    } else {
        Err(format!(
            "{path}:{}: métrica fora do domínio",
            zero_based_line + 1
        ))
    }
}

fn pair_samples(
    samples: &[Sample],
    expected_repetitions: usize,
) -> Result<BTreeMap<String, Vec<Pair>>, String> {
    let mut grouped = BTreeMap::<(String, usize), BTreeMap<String, &Sample>>::new();
    for sample in samples {
        if sample.status != "OK" {
            return Err(format!(
                "{} repetição {} terminou com {}",
                sample.benchmark, sample.repetition, sample.status
            ));
        }
        let profiles = grouped
            .entry((sample.benchmark.clone(), sample.repetition))
            .or_default();
        if profiles.insert(sample.profile.clone(), sample).is_some() {
            return Err(format!(
                "amostra duplicada para {} repetição {} perfil {}",
                sample.benchmark, sample.repetition, sample.profile
            ));
        }
    }

    let mut result = BTreeMap::<String, Vec<Pair>>::new();
    for ((benchmark, repetition), profiles) in grouped {
        let control = profiles
            .get("none")
            .ok_or_else(|| format!("{benchmark} repetição {repetition}: controle ausente"))?;
        let candidate = profiles
            .get("safe")
            .ok_or_else(|| format!("{benchmark} repetição {repetition}: candidato ausente"))?;
        if control.checksum != candidate.checksum {
            return Err(format!(
                "{benchmark} repetição {repetition}: checksums divergem ({} != {})",
                control.checksum, candidate.checksum
            ));
        }
        result.entry(benchmark).or_default().push(Pair {
            control_wall: control.wall,
            candidate_wall: candidate.wall,
            control_cpu: control.cpu,
            candidate_cpu: candidate.cpu,
            control_rss: control.rss,
            candidate_rss: candidate.rss,
        });
    }
    for (benchmark, values) in &result {
        if values.len() != expected_repetitions {
            return Err(format!(
                "{benchmark}: esperadas {expected_repetitions} repetições, encontradas {}",
                values.len()
            ));
        }
    }
    Ok(result)
}

fn ratio(candidate: f64, control: f64) -> f64 {
    if control == 0.0 {
        if candidate == 0.0 {
            1.0
        } else {
            f64::INFINITY
        }
    } else {
        candidate / control
    }
}

fn estimate(values: &[f64], seed: u64) -> Estimate {
    let point = median(values);
    let deviations = values
        .iter()
        .map(|value| (value - point).abs())
        .collect::<Vec<_>>();
    let mad = median(&deviations);
    let mut state = seed;
    let mut bootstrap = Vec::with_capacity(10_000);
    for _ in 0..10_000 {
        let selected = (0..values.len())
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                values[(state as usize) % values.len()]
            })
            .collect::<Vec<_>>();
        bootstrap.push(median(&selected));
    }
    bootstrap.sort_by(f64::total_cmp);
    Estimate {
        median: point,
        mad,
        low: percentile(&bootstrap, 0.025),
        high: percentile(&bootstrap, 0.975),
    }
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    }
}

fn percentile(sorted: &[f64], quantile: f64) -> f64 {
    let index = ((sorted.len() - 1) as f64 * quantile).round() as usize;
    sorted[index]
}

fn confirmed_regression(estimate: Estimate) -> bool {
    estimate.median > 1.01 && estimate.low > 1.0
}

fn evaluate_gate(
    pairs: &BTreeMap<String, Vec<Pair>>,
    repetitions: usize,
    scale: usize,
    partial: bool,
) -> bool {
    if partial || repetitions < 7 || scale != 25 || pairs.len() != 30 {
        return false;
    }
    let all = aggregate_pairs(pairs);
    let wall = estimate(
        &all.iter().map(|pair| pair.wall_ratio()).collect::<Vec<_>>(),
        1,
    );
    let cpu = estimate(
        &all.iter().map(|pair| pair.cpu_ratio()).collect::<Vec<_>>(),
        2,
    );
    if wall.median > 0.97 || wall.median > 1.0 || cpu.median > 1.0 {
        return false;
    }
    for values in pairs.values() {
        let wall = estimate(
            &values
                .iter()
                .map(|pair| pair.wall_ratio())
                .collect::<Vec<_>>(),
            3,
        );
        let cpu = estimate(
            &values
                .iter()
                .map(|pair| pair.cpu_ratio())
                .collect::<Vec<_>>(),
            4,
        );
        if wall.median > 1.03
            || cpu.median > 1.03
            || confirmed_regression(wall)
            || confirmed_regression(cpu)
        {
            return false;
        }
    }
    for values in chapter_pairs(pairs).values() {
        let wall = estimate(
            &values
                .iter()
                .map(|pair| pair.wall_ratio())
                .collect::<Vec<_>>(),
            5,
        );
        let cpu = estimate(
            &values
                .iter()
                .map(|pair| pair.cpu_ratio())
                .collect::<Vec<_>>(),
            6,
        );
        if confirmed_regression(wall) || confirmed_regression(cpu) {
            return false;
        }
    }
    true
}

fn aggregate_pairs(pairs: &BTreeMap<String, Vec<Pair>>) -> Vec<Pair> {
    let repetitions = pairs.values().next().map_or(0, Vec::len);
    (0..repetitions)
        .map(|index| {
            let values = pairs.values().map(|pairs| pairs[index]).collect::<Vec<_>>();
            Pair {
                control_wall: values.iter().map(|pair| pair.control_wall).sum(),
                candidate_wall: values.iter().map(|pair| pair.candidate_wall).sum(),
                control_cpu: values.iter().map(|pair| pair.control_cpu).sum(),
                candidate_cpu: values.iter().map(|pair| pair.candidate_cpu).sum(),
                control_rss: values.iter().map(|pair| pair.control_rss).sum(),
                candidate_rss: values.iter().map(|pair| pair.candidate_rss).sum(),
            }
        })
        .collect()
}

fn chapter_pairs(pairs: &BTreeMap<String, Vec<Pair>>) -> BTreeMap<String, Vec<Pair>> {
    let mut chapters = BTreeMap::<String, Vec<Vec<Pair>>>::new();
    for (benchmark, values) in pairs {
        let chapter = benchmark.split('/').next().unwrap_or(benchmark).to_string();
        chapters.entry(chapter).or_default().push(values.clone());
    }
    chapters
        .into_iter()
        .map(|(chapter, cases)| {
            let repetitions = cases.first().map_or(0, Vec::len);
            let values = (0..repetitions)
                .map(|index| {
                    let samples = cases.iter().map(|case| case[index]).collect::<Vec<_>>();
                    Pair {
                        control_wall: samples.iter().map(|pair| pair.control_wall).sum(),
                        candidate_wall: samples.iter().map(|pair| pair.candidate_wall).sum(),
                        control_cpu: samples.iter().map(|pair| pair.control_cpu).sum(),
                        candidate_cpu: samples.iter().map(|pair| pair.candidate_cpu).sum(),
                        control_rss: samples.iter().map(|pair| pair.control_rss).sum(),
                        candidate_rss: samples.iter().map(|pair| pair.candidate_rss).sum(),
                    }
                })
                .collect();
            (chapter, values)
        })
        .collect()
}

fn render_report(
    pairs: &BTreeMap<String, Vec<Pair>>,
    repetitions: usize,
    scale: usize,
    partial: bool,
) -> String {
    let mut output = String::new();
    let chapters = chapter_pairs(pairs);
    let aggregate = aggregate_pairs(pairs);
    let aggregate_wall = estimate(
        &aggregate
            .iter()
            .map(|pair| pair.wall_ratio())
            .collect::<Vec<_>>(),
        10,
    );
    let aggregate_cpu = estimate(
        &aggregate
            .iter()
            .map(|pair| pair.cpu_ratio())
            .collect::<Vec<_>>(),
        11,
    );
    let gate = evaluate_gate(pairs, repetitions, scale, partial);
    output.push_str("# Cormen native IR A/B\n\n");
    output.push_str(&format!(
        "- Control: `--ir-opt none --opt-level none`\n\
         - Candidate: `--ir-opt safe --opt-level none`\n\
         - Repetitions: {repetitions} paired and alternating\n\
         - Scale: {scale}\n\
         - Cases: {}\n\
         - Gate: **{}**\n\n",
        pairs.len(),
        if gate { "PASS" } else { "NOT PROMOTED" }
    ));
    if partial {
        output.push_str(
            "> This is a chapter-scoped diagnostic run and cannot promote the profile.\n\n",
        );
    } else if repetitions < 7 || scale != 25 || pairs.len() != 30 {
        output.push_str(
            "> This run does not meet the ADR-0014 sample, scale, or case-count contract.\n\n",
        );
    }
    output.push_str(
        "| Scope | Wall candidate/control | Wall MAD | Wall 95% CI | CPU candidate/control | CPU MAD | CPU 95% CI |\n\
         | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n",
    );
    push_estimate_row(&mut output, "Aggregate", aggregate_wall, aggregate_cpu);
    for (chapter, values) in &chapters {
        push_estimate_row(
            &mut output,
            chapter,
            estimate(
                &values
                    .iter()
                    .map(|pair| pair.wall_ratio())
                    .collect::<Vec<_>>(),
                stable_seed(chapter, 20),
            ),
            estimate(
                &values
                    .iter()
                    .map(|pair| pair.cpu_ratio())
                    .collect::<Vec<_>>(),
                stable_seed(chapter, 21),
            ),
        );
    }
    output.push_str("\n## Cases\n\n");
    output.push_str(
        "| Benchmark | Wall candidate/control | Wall 95% CI | CPU candidate/control | CPU 95% CI | RSS none KiB | RSS safe KiB |\n\
         | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n",
    );
    for (benchmark, values) in pairs {
        let wall = estimate(
            &values
                .iter()
                .map(|pair| pair.wall_ratio())
                .collect::<Vec<_>>(),
            stable_seed(benchmark, 30),
        );
        let cpu = estimate(
            &values
                .iter()
                .map(|pair| pair.cpu_ratio())
                .collect::<Vec<_>>(),
            stable_seed(benchmark, 31),
        );
        output.push_str(&format!(
            "| `{benchmark}` | {:.4} | {:.4}–{:.4} | {:.4} | {:.4}–{:.4} | {} | {} |\n",
            wall.median,
            wall.low,
            wall.high,
            cpu.median,
            cpu.low,
            cpu.high,
            median_u64(values.iter().map(|pair| pair.control_rss)),
            median_u64(values.iter().map(|pair| pair.candidate_rss)),
        ));
    }
    output
}

fn push_estimate_row(output: &mut String, scope: &str, wall: Estimate, cpu: Estimate) {
    output.push_str(&format!(
        "| {scope} | {:.4} | {:.4} | {:.4}–{:.4} | {:.4} | {:.4} | {:.4}–{:.4} |\n",
        wall.median, wall.mad, wall.low, wall.high, cpu.median, cpu.mad, cpu.low, cpu.high
    ));
}

fn median_u64(values: impl Iterator<Item = u64>) -> u64 {
    let mut values = values.collect::<Vec<_>>();
    values.sort_unstable();
    values[values.len() / 2]
}

fn stable_seed(text: &str, salt: u64) -> u64 {
    text.bytes()
        .fold(1_469_598_103_934_665_603_u64 ^ salt, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(1_099_511_628_211)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn median_and_mad_are_deterministic() {
        let result = estimate(&[1.0, 0.9, 1.1, 1.0, 1.0, 0.8, 1.2], 42);
        assert_eq!(result.median, 1.0);
        assert!((result.mad - 0.1).abs() < f64::EPSILON);
        assert!(result.low <= result.median);
        assert!(result.high >= result.median);
    }

    #[test]
    fn pairs_require_matching_checksums() {
        let samples = vec![
            Sample {
                benchmark: "case".to_string(),
                repetition: 1,
                profile: "none".to_string(),
                wall: 1.0,
                cpu: 1.0,
                rss: 1,
                checksum: "10".to_string(),
                status: "OK".to_string(),
            },
            Sample {
                benchmark: "case".to_string(),
                repetition: 1,
                profile: "safe".to_string(),
                wall: 0.9,
                cpu: 0.9,
                rss: 1,
                checksum: "11".to_string(),
                status: "OK".to_string(),
            },
        ];
        let error = pair_samples(&samples, 1).expect_err("checksum mismatch");
        assert!(error.contains("checksums divergem"));
    }

    #[test]
    fn partial_runs_never_promote_the_profile() {
        let pairs = BTreeMap::from([(
            "01/case".to_string(),
            vec![
                Pair {
                    control_wall: 1.0,
                    candidate_wall: 0.5,
                    control_cpu: 1.0,
                    candidate_cpu: 0.5,
                    control_rss: 10,
                    candidate_rss: 10,
                };
                7
            ],
        )]);
        assert!(!evaluate_gate(&pairs, 7, 25, true));
    }

    #[test]
    fn chapter_names_are_collected_stably() {
        let pairs = BTreeMap::from([
            ("02/b".to_string(), Vec::new()),
            ("01/a".to_string(), Vec::new()),
        ]);
        assert_eq!(
            chapter_pairs(&pairs)
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["01".to_string(), "02".to_string()])
        );
    }
}
