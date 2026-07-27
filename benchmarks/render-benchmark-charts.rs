use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const WIDTH: i32 = 1500;
const ROW_HEIGHT: i32 = 24;
const HEADER_HEIGHT: i32 = 196;
const FOOTER_HEIGHT: i32 = 54;
const PLOT_LEFT: f64 = 570.0;
const PLOT_WIDTH: f64 = 720.0;
const VALUE_X: i32 = 1310;

#[derive(Debug)]
struct Benchmark {
    name: String,
    native_wall: f64,
    native_cpu: f64,
    native_rss_mib: f64,
    clojure_wall: f64,
    clojure_cpu: f64,
    clojure_rss_mib: f64,
}

#[derive(Clone, Copy)]
enum TimeMetric {
    Wall,
    Cpu,
}

impl TimeMetric {
    fn title(self) -> &'static str {
        match self {
            Self::Wall => "tempo de parede",
            Self::Cpu => "tempo total de CPU",
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::Wall => "wall-time.svg",
            Self::Cpu => "cpu-time.svg",
        }
    }

    fn values(self, benchmark: &Benchmark) -> (f64, f64) {
        match self {
            Self::Wall => (benchmark.native_wall, benchmark.clojure_wall),
            Self::Cpu => (benchmark.native_cpu, benchmark.clojure_cpu),
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let arguments: Vec<String> = env::args().collect();
    if !(3..=4).contains(&arguments.len()) {
        return Err("Uso: render-benchmark-charts CSV DIRETORIO [TITULO]".to_owned());
    }

    let csv_path = Path::new(&arguments[1]);
    let output_dir = Path::new(&arguments[2]);
    let suite_title = arguments.get(3).map_or("Benchmarks", String::as_str);
    let benchmarks = load_benchmarks(csv_path)?;

    fs::create_dir_all(output_dir)
        .map_err(|error| format!("Não foi possível criar {}: {error}", output_dir.display()))?;

    for metric in [TimeMetric::Wall, TimeMetric::Cpu] {
        write_svg(
            &output_dir.join(metric.file_name()),
            &render_ratio_chart(&benchmarks, suite_title, metric),
        )?;
    }
    write_svg(
        &output_dir.join("memory-rss.svg"),
        &render_memory_chart(&benchmarks, suite_title),
    )?;

    println!("Gráficos gerados em {}", output_dir.display());
    Ok(())
}

fn load_benchmarks(path: &Path) -> Result<Vec<Benchmark>, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("Não foi possível ler {}: {error}", path.display()))?;
    let mut lines = contents.lines();
    let header = lines
        .next()
        .ok_or_else(|| format!("CSV comparativo vazio: {}", path.display()))?;
    let header_fields = parse_csv_line(header)?;

    let required_columns = [
        (0, "benchmark"),
        (3, "native_wall_time_s"),
        (6, "native_cpu_total_s"),
        (8, "native_max_rss_kb"),
        (11, "clojure_wall_time_s"),
        (14, "clojure_cpu_total_s"),
        (16, "clojure_max_rss_kb"),
        (22, "status"),
    ];
    for (index, expected) in required_columns {
        if header_fields.get(index).map(String::as_str) != Some(expected) {
            return Err(format!(
                "Cabeçalho comparativo incompatível em {}: coluna {} deve ser {expected}",
                path.display(),
                index + 1
            ));
        }
    }

    let mut benchmarks = Vec::new();
    for (offset, line) in lines.enumerate() {
        let line_number = offset + 2;
        if line.trim().is_empty() {
            continue;
        }
        let fields = parse_csv_line(line)
            .map_err(|error| format!("{}:{line_number}: {error}", path.display()))?;
        benchmarks.push(parse_benchmark(&fields, path, line_number)?);
    }

    if benchmarks.is_empty() {
        return Err(format!("CSV comparativo sem casos: {}", path.display()));
    }
    Ok(benchmarks)
}

fn parse_benchmark(
    fields: &[String],
    path: &Path,
    line_number: usize,
) -> Result<Benchmark, String> {
    if fields.len() != 23 {
        return Err(format!(
            "{}:{line_number}: esperadas 23 colunas, encontradas {}",
            path.display(),
            fields.len()
        ));
    }

    let name = fields[0].clone();
    if fields[22] != "OK" {
        return Err(format!(
            "{}:{line_number}: {name} possui status {}",
            path.display(),
            fields[22]
        ));
    }
    if fields[17] != fields[18] {
        return Err(format!(
            "{}:{line_number}: {name} possui checksums divergentes",
            path.display()
        ));
    }

    let native_rss_kib = parse_metric(fields, 8, "native_max_rss_kb", path, line_number)?;
    let clojure_rss_kib = parse_metric(fields, 16, "clojure_max_rss_kb", path, line_number)?;
    if native_rss_kib <= 0.0 || clojure_rss_kib <= 0.0 {
        return Err(format!(
            "{}:{line_number}: {name} possui RSS não positivo",
            path.display()
        ));
    }

    Ok(Benchmark {
        name,
        native_wall: parse_metric(fields, 3, "native_wall_time_s", path, line_number)?,
        native_cpu: parse_metric(fields, 6, "native_cpu_total_s", path, line_number)?,
        native_rss_mib: native_rss_kib / 1024.0,
        clojure_wall: parse_metric(fields, 11, "clojure_wall_time_s", path, line_number)?,
        clojure_cpu: parse_metric(fields, 14, "clojure_cpu_total_s", path, line_number)?,
        clojure_rss_mib: clojure_rss_kib / 1024.0,
    })
}

fn parse_metric(
    fields: &[String],
    index: usize,
    name: &str,
    path: &Path,
    line_number: usize,
) -> Result<f64, String> {
    let value = fields[index].parse::<f64>().map_err(|error| {
        format!(
            "{}:{line_number}: valor inválido em {name}: {} ({error})",
            path.display(),
            fields[index]
        )
    })?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!(
            "{}:{line_number}: valor inválido em {name}: {}",
            path.display(),
            fields[index]
        ));
    }
    Ok(value)
}

fn parse_csv_line(line: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut characters = line.chars().peekable();
    let mut quoted = false;

    while let Some(character) = characters.next() {
        match character {
            '"' if quoted && characters.peek() == Some(&'"') => {
                characters.next();
                field.push('"');
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(std::mem::take(&mut field));
            }
            _ => field.push(character),
        }
    }
    if quoted {
        return Err("campo CSV com aspas não terminadas".to_owned());
    }
    fields.push(field);
    Ok(fields)
}

fn write_svg(path: &Path, contents: &str) -> Result<(), String> {
    let temporary_path = temporary_svg_path(path);
    fs::write(&temporary_path, contents).map_err(|error| {
        format!(
            "Não foi possível escrever {}: {error}",
            temporary_path.display()
        )
    })?;
    fs::rename(&temporary_path, path).map_err(|error| {
        let _ = fs::remove_file(&temporary_path);
        format!("Não foi possível publicar {}: {error}", path.display())
    })
}

fn temporary_svg_path(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("chart.svg");
    path.with_file_name(format!(".{file_name}.tmp"))
}

fn render_ratio_chart(benchmarks: &[Benchmark], suite_title: &str, metric: TimeMetric) -> String {
    let height = chart_height(benchmarks.len());
    let groups = chapter_groups(benchmarks);
    let values: Vec<(f64, f64)> = benchmarks
        .iter()
        .map(|benchmark| metric.values(benchmark))
        .collect();
    let total_native: f64 = values.iter().map(|(native, _)| native).sum();
    let total_clojure: f64 = values.iter().map(|(_, clojure)| clojure).sum();
    let (native_wins, clojure_wins, ties) = winner_counts(&values);
    let escaped_suite = xml_escape(suite_title);
    let escaped_metric = xml_escape(metric.title());
    let mut svg = String::new();

    writeln!(svg, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
    writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{height}" viewBox="0 0 {WIDTH} {height}" role="img" aria-labelledby="chart-title chart-desc">"#
    )
    .unwrap();
    writeln!(
        svg,
        r#"  <title id="chart-title">{escaped_suite} — {escaped_metric}</title>"#
    )
    .unwrap();
    svg.push_str(
        "  <desc id=\"chart-desc\">Comparação por caso. Barras à esquerda favorecem Clojure/JVM; barras à direita favorecem o executável nativo.</desc>\n",
    );
    push_styles(&mut svg);
    writeln!(
        svg,
        r##"  <rect width="{WIDTH}" height="{height}" rx="16" fill="#0f172a"/>"##
    )
    .unwrap();
    writeln!(
        svg,
        r#"  <text x="24" y="42" class="title">{escaped_suite} — {escaped_metric}</text>"#
    )
    .unwrap();
    svg.push_str(
        "  <text x=\"24\" y=\"70\" class=\"subtitle\">Razão = Clojure/JVM ÷ nativo. Quanto maior a barra à direita, maior a vantagem nativa.</text>\n",
    );
    writeln!(
        svg,
        r#"  <text x="24" y="101" class="summary">Total nativo: {total_native:.2} s · JVM: {total_clojure:.2} s · Vitórias: nativo {native_wins}, JVM {clojure_wins}, empates {ties}</text>"#
    )
    .unwrap();
    svg.push_str(
        "  <rect x=\"24\" y=\"122\" width=\"14\" height=\"14\" rx=\"3\" fill=\"#34d399\"/>\n\
         <text x=\"46\" y=\"134\" class=\"subtitle\">nativo mais rápido</text>\n\
         <rect x=\"205\" y=\"122\" width=\"14\" height=\"14\" rx=\"3\" fill=\"#fb923c\"/>\n\
         <text x=\"227\" y=\"134\" class=\"subtitle\">JVM mais rápida</text>\n\
         <text x=\"24\" y=\"174\" class=\"axis\">Caso</text>\n",
    );
    writeln!(
        svg,
        r#"  <text x="{:.0}" y="174" class="axis" text-anchor="middle">JVM mais rápida ← paridade → nativo mais rápido</text>"#,
        ratio_x(1.0)
    )
    .unwrap();
    writeln!(
        svg,
        r#"  <text x="{VALUE_X}" y="174" class="axis">Vantagem</text>"#
    )
    .unwrap();
    push_ratio_axis(&mut svg, height);

    for (row_index, benchmark) in benchmarks.iter().enumerate() {
        let (native, clojure) = values[row_index];
        let limited = native <= 0.0 && clojure > 0.0;
        let ratio = if limited {
            clojure / 0.01
        } else if native > 0.0 && clojure > 0.0 {
            clojure / native
        } else {
            1.0
        };
        let endpoint = ratio_x(ratio);
        let parity = ratio_x(1.0);
        let bar_x = endpoint.min(parity);
        let bar_width = (endpoint - parity).abs();
        let color = if ratio < 1.0 { "#fb923c" } else { "#34d399" };
        let row_top = HEADER_HEIGHT + row_index as i32 * ROW_HEIGHT;
        let row_center = f64::from(row_top) + f64::from(ROW_HEIGHT) / 2.0;
        let row_fill = if groups[row_index] % 2 == 1 {
            "#111c2e"
        } else {
            "#0f172a"
        };
        let escaped_name = xml_escape(&benchmark.name);
        let advantage = advantage_label(ratio, limited);

        push_row_background(&mut svg, row_top, row_fill);
        writeln!(
            svg,
            r#"  <text x="24" y="{row_center:.1}" class="label" dominant-baseline="middle">{escaped_name}</text>"#
        )
        .unwrap();
        if bar_width < 2.0 {
            write!(
                svg,
                r##"  <circle cx="{parity:.1}" cy="{row_center:.1}" r="4" fill="#94a3b8">"##
            )
            .unwrap();
        } else {
            write!(
                svg,
                r#"  <rect x="{bar_x:.1}" y="{:.1}" width="{bar_width:.1}" height="12" rx="4" fill="{color}">"#,
                row_center - 6.0
            )
            .unwrap();
        }
        write!(
            svg,
            "<title>{escaped_name}: nativo {native:.2} s; JVM {clojure:.2} s; {advantage}</title>"
        )
        .unwrap();
        if bar_width < 2.0 {
            svg.push_str("</circle>\n");
        } else {
            svg.push_str("</rect>\n");
        }
        writeln!(
            svg,
            r#"  <text x="{VALUE_X}" y="{row_center:.1}" class="value" dominant-baseline="middle">{advantage}</text>"#
        )
        .unwrap();
    }

    writeln!(
        svg,
        r#"  <text x="24" y="{}" class="footer">Menor é melhor. Escala log₂; barras limitadas às bordas do gráfico. * Valor nativo abaixo da resolução de 0,01 s do runner.</text>"#,
        height - 20
    )
    .unwrap();
    svg.push_str("</svg>\n");
    svg
}

fn render_memory_chart(benchmarks: &[Benchmark], suite_title: &str) -> String {
    let height = chart_height(benchmarks.len());
    let groups = chapter_groups(benchmarks);
    let native_values: Vec<f64> = benchmarks.iter().map(|row| row.native_rss_mib).collect();
    let clojure_values: Vec<f64> = benchmarks.iter().map(|row| row.clojure_rss_mib).collect();
    let paired_values: Vec<(f64, f64)> = native_values
        .iter()
        .copied()
        .zip(clojure_values.iter().copied())
        .collect();
    let (native_wins, clojure_wins, ties) = winner_counts(&paired_values);
    let median_native = median(&native_values);
    let median_clojure = median(&clojure_values);
    let escaped_suite = xml_escape(suite_title);
    let mut svg = String::new();

    writeln!(svg, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
    writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{height}" viewBox="0 0 {WIDTH} {height}" role="img" aria-labelledby="chart-title chart-desc">"#
    )
    .unwrap();
    writeln!(
        svg,
        r#"  <title id="chart-title">{escaped_suite} — pico de memória RSS</title>"#
    )
    .unwrap();
    svg.push_str(
        "  <desc id=\"chart-desc\">Comparação do pico de memória residente em MiB por caso, em escala logarítmica.</desc>\n",
    );
    push_styles(&mut svg);
    writeln!(
        svg,
        r##"  <rect width="{WIDTH}" height="{height}" rx="16" fill="#0f172a"/>"##
    )
    .unwrap();
    writeln!(
        svg,
        r#"  <text x="24" y="42" class="title">{escaped_suite} — pico de memória RSS</text>"#
    )
    .unwrap();
    svg.push_str(
        "  <text x=\"24\" y=\"70\" class=\"subtitle\">Valores absolutos por processo; pontos mais à esquerda consomem menos memória.</text>\n",
    );
    writeln!(
        svg,
        r#"  <text x="24" y="101" class="summary">Mediana nativa: {median_native:.1} MiB · JVM: {median_clojure:.1} MiB · Menor RSS: nativo {native_wins}, JVM {clojure_wins}, empates {ties}</text>"#
    )
    .unwrap();
    svg.push_str(
        "  <circle cx=\"31\" cy=\"129\" r=\"6\" fill=\"#38bdf8\"/>\n\
         <text x=\"46\" y=\"134\" class=\"subtitle\">nativo</text>\n\
         <path d=\"M 201 122 L 208 129 L 201 136 L 194 129 Z\" fill=\"#c084fc\"/>\n\
         <text x=\"218\" y=\"134\" class=\"subtitle\">Clojure/JVM</text>\n\
         <text x=\"24\" y=\"174\" class=\"axis\">Caso</text>\n\
         <text x=\"930\" y=\"174\" class=\"axis\" text-anchor=\"middle\">Pico de RSS em MiB (escala log₂)</text>\n",
    );
    writeln!(
        svg,
        r#"  <text x="{VALUE_X}" y="174" class="axis">Menor consumo</text>"#
    )
    .unwrap();
    push_memory_axis(&mut svg, height);

    for (row_index, benchmark) in benchmarks.iter().enumerate() {
        let native = benchmark.native_rss_mib;
        let clojure = benchmark.clojure_rss_mib;
        let ratio = clojure / native;
        let native_x = memory_x(native);
        let clojure_x = memory_x(clojure);
        let row_top = HEADER_HEIGHT + row_index as i32 * ROW_HEIGHT;
        let row_center = f64::from(row_top) + f64::from(ROW_HEIGHT) / 2.0;
        let row_fill = if groups[row_index] % 2 == 1 {
            "#111c2e"
        } else {
            "#0f172a"
        };
        let escaped_name = xml_escape(&benchmark.name);

        push_row_background(&mut svg, row_top, row_fill);
        writeln!(
            svg,
            r#"  <text x="24" y="{row_center:.1}" class="label" dominant-baseline="middle">{escaped_name}</text>"#
        )
        .unwrap();
        writeln!(
            svg,
            r##"  <line x1="{native_x:.1}" y1="{row_center:.1}" x2="{clojure_x:.1}" y2="{row_center:.1}" stroke="#64748b" stroke-width="2" opacity="0.75"/>"##
        )
        .unwrap();
        writeln!(
            svg,
            r##"  <circle cx="{native_x:.1}" cy="{row_center:.1}" r="5" fill="#38bdf8"><title>{escaped_name}: nativo {native:.1} MiB</title></circle>"##
        )
        .unwrap();
        writeln!(
            svg,
            r##"  <path d="M {clojure_x:.1} {:.1} L {:.1} {row_center:.1} L {clojure_x:.1} {:.1} L {:.1} {row_center:.1} Z" fill="#c084fc"><title>{escaped_name}: JVM {clojure:.1} MiB</title></path>"##,
            row_center - 6.0,
            clojure_x + 6.0,
            row_center + 6.0,
            clojure_x - 6.0
        )
        .unwrap();
        writeln!(
            svg,
            r#"  <text x="{VALUE_X}" y="{row_center:.1}" class="value" dominant-baseline="middle">{}</text>"#,
            memory_advantage_label(ratio)
        )
        .unwrap();
    }

    writeln!(
        svg,
        r#"  <text x="24" y="{}" class="footer">Menor é melhor. A coluna final mostra quantas vezes o vencedor consumiu menos RSS no caso.</text>"#,
        height - 20
    )
    .unwrap();
    svg.push_str("</svg>\n");
    svg
}

fn push_styles(svg: &mut String) {
    svg.push_str(
        "  <style>\n\
         .title { fill: #f8fafc; font: 700 26px system-ui, sans-serif; }\n\
         .subtitle { fill: #cbd5e1; font: 14px system-ui, sans-serif; }\n\
         .summary { fill: #e2e8f0; font: 600 14px system-ui, sans-serif; }\n\
         .label { fill: #dbeafe; font: 10px ui-monospace, SFMono-Regular, Consolas, monospace; }\n\
         .value { fill: #e2e8f0; font: 600 11px system-ui, sans-serif; }\n\
         .axis { fill: #94a3b8; font: 11px system-ui, sans-serif; }\n\
         .footer { fill: #94a3b8; font: 11px system-ui, sans-serif; }\n\
         </style>\n",
    );
}

fn push_ratio_axis(svg: &mut String, height: i32) {
    for exponent in -6_i32..=6 {
        let x = ratio_x(2_f64.powi(exponent));
        let (stroke, opacity, label) = if exponent == 0 {
            ("#e2e8f0", "0.90", Some("1×".to_owned()))
        } else {
            let label = if exponent % 2 == 0 {
                if exponent < 0 {
                    Some(format!("JVM {}×", 2_i32.pow((-exponent) as u32)))
                } else {
                    Some(format!("Nativo {}×", 2_i32.pow(exponent as u32)))
                }
            } else {
                None
            };
            (
                "#334155",
                if exponent % 2 == 0 { "0.75" } else { "0.35" },
                label,
            )
        };
        writeln!(
            svg,
            r#"  <line x1="{x:.1}" y1="180" x2="{x:.1}" y2="{}" stroke="{stroke}" opacity="{opacity}"/>"#,
            height - FOOTER_HEIGHT + 4
        )
        .unwrap();
        if let Some(label) = label {
            writeln!(
                svg,
                r#"  <text x="{x:.1}" y="190" class="axis" text-anchor="middle">{label}</text>"#
            )
            .unwrap();
        }
    }
}

fn push_memory_axis(svg: &mut String, height: i32) {
    for exponent in -1_i32..=11 {
        let value = 2_f64.powi(exponent);
        let x = memory_x(value);
        let opacity = if exponent >= 0 { "0.75" } else { "0.35" };
        writeln!(
            svg,
            r##"  <line x1="{x:.1}" y1="180" x2="{x:.1}" y2="{}" stroke="#334155" opacity="{opacity}"/>"##,
            height - FOOTER_HEIGHT + 4
        )
        .unwrap();
        if exponent >= 0 {
            writeln!(
                svg,
                r#"  <text x="{x:.1}" y="190" class="axis" text-anchor="middle">{value:.0}</text>"#
            )
            .unwrap();
        }
    }
}

fn push_row_background(svg: &mut String, row_top: i32, fill: &str) {
    writeln!(
        svg,
        r#"  <rect x="12" y="{row_top}" width="1476" height="{ROW_HEIGHT}" fill="{fill}"/>"#
    )
    .unwrap();
    writeln!(
        svg,
        r##"  <line x1="12" y1="{row_top}" x2="1488" y2="{row_top}" stroke="#1e293b"/>"##
    )
    .unwrap();
}

fn chart_height(case_count: usize) -> i32 {
    HEADER_HEIGHT + case_count as i32 * ROW_HEIGHT + FOOTER_HEIGHT
}

fn chapter_groups(benchmarks: &[Benchmark]) -> Vec<usize> {
    let mut groups = Vec::with_capacity(benchmarks.len());
    let mut previous = "";
    let mut group = 0;
    for benchmark in benchmarks {
        let chapter = benchmark.name.split('/').next().unwrap_or(&benchmark.name);
        if chapter != previous {
            group += 1;
            previous = chapter;
        }
        groups.push(group);
    }
    groups
}

fn winner_counts(values: &[(f64, f64)]) -> (usize, usize, usize) {
    values.iter().fold(
        (0, 0, 0),
        |(native_wins, clojure_wins, ties), (native, clojure)| {
            if native < clojure {
                (native_wins + 1, clojure_wins, ties)
            } else if native > clojure {
                (native_wins, clojure_wins + 1, ties)
            } else {
                (native_wins, clojure_wins, ties + 1)
            }
        },
    )
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2]
    } else {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    }
}

fn ratio_x(ratio: f64) -> f64 {
    let exponent = ratio.log2().clamp(-6.0, 6.0);
    PLOT_LEFT + (exponent + 6.0) * PLOT_WIDTH / 12.0
}

fn memory_x(memory_mib: f64) -> f64 {
    let exponent = memory_mib.log2().clamp(-1.0, 11.0);
    PLOT_LEFT + (exponent + 1.0) * PLOT_WIDTH / 12.0
}

fn advantage_label(ratio: f64, limited: bool) -> String {
    if limited {
        format!("Nativo >{ratio:.0}×*")
    } else if ratio > 1.005 {
        format!("Nativo {ratio:.2}×")
    } else if ratio < 0.995 {
        format!("JVM {:.2}×", 1.0 / ratio)
    } else {
        "empate".to_owned()
    }
}

fn memory_advantage_label(ratio: f64) -> String {
    if ratio > 1.005 {
        format!("Nativo {ratio:.1}×")
    } else if ratio < 0.995 {
        format!("JVM {:.1}×", 1.0 / ratio)
    } else {
        "empate".to_owned()
    }
}

fn xml_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn benchmark(name: &str) -> Benchmark {
        Benchmark {
            name: name.to_owned(),
            native_wall: 1.0,
            native_cpu: 0.8,
            native_rss_mib: 16.0,
            clojure_wall: 2.0,
            clojure_cpu: 1.6,
            clojure_rss_mib: 256.0,
        }
    }

    #[test]
    fn parses_quoted_csv_fields() {
        let fields = parse_csv_line(r#"one,"two,three","four""five""#).unwrap();
        assert_eq!(fields, ["one", "two,three", "four\"five"]);
    }

    #[test]
    fn rejects_unterminated_csv_quotes() {
        assert!(parse_csv_line(r#"one,"two"#).is_err());
    }

    #[test]
    fn escapes_xml_text() {
        assert_eq!(
            xml_escape(r#"<case kind="a&b">"#),
            "&lt;case kind=&quot;a&amp;b&quot;&gt;"
        );
    }

    #[test]
    fn labels_both_sides_of_parity() {
        assert_eq!(advantage_label(4.0, false), "Nativo 4.00×");
        assert_eq!(advantage_label(0.25, false), "JVM 4.00×");
        assert_eq!(advantage_label(36.0, true), "Nativo >36×*");
    }

    #[test]
    fn renders_accessible_ratio_chart_with_case_data() {
        let chart = render_ratio_chart(
            &[benchmark("01-group/01-case.clj")],
            "Suite",
            TimeMetric::Wall,
        );
        assert!(chart.contains(r#"role="img""#));
        assert!(chart.contains("01-group/01-case.clj"));
        assert!(chart.contains("Total nativo: 1.00 s"));
        assert!(chart.contains("Nativo 2.00×"));
    }

    #[test]
    fn renders_absolute_memory_values() {
        let chart = render_memory_chart(&[benchmark("01-group/01-case.clj")], "Suite");
        assert!(chart.contains("nativo 16.0 MiB"));
        assert!(chart.contains("JVM 256.0 MiB"));
        assert!(chart.contains("Nativo 16.0×"));
    }
}
