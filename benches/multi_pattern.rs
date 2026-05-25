use std::hint::black_box;
use std::path::Path;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use glob::Pattern;
use globset::{Glob, GlobSet, GlobSetBuilder};
use pcre2::bytes::{Regex as Pcre2Regex, RegexBuilder as Pcre2RegexBuilder};
use regex::bytes::RegexSet;
use wax::Program;

const INCLUDE_PATTERNS: &[&str] = &[
    "src/**/*.rs",
    "tests/**/*.rs",
    "tests/**/*_test.rs",
    "benches/**/*.rs",
    "examples/**/*.rs",
    "crates/*/src/**/*.rs",
    "packages/*/src/**/*.ts",
    "packages/*/src/**/*.tsx",
    "assets/**/*.png",
    "assets/**/*.jpg",
    "assets/**/*.svg",
    "docs/**/*.md",
    "**/Cargo.toml",
    "**/README.md",
    "scripts/*.sh",
    "config/*.yaml",
];

const EXCLUDE_PATTERNS: &[&str] = &[
    "target/**",
    "**/node_modules/**",
    "**/dist/**",
    "**/.git/**",
    "**/*.generated.rs",
    "**/*.snap",
    "**/*.lock",
    "assets/**/cache/**",
    "docs/**/drafts/**",
    "tmp/**",
];

const TYPESCRIPT_INCLUDE_PATTERNS: &[&str] = &["**/**.ts"];
const TYPESCRIPT_EXCLUDE_PATTERNS: &[&str] = &["foo/*.ts"];

fn corpus(size: usize) -> Vec<String> {
    let roots = [
        "src",
        "tests",
        "benches",
        "examples",
        "crates/core/src",
        "crates/cli/src",
        "packages/web/src",
        "packages/web/node_modules/react",
        "packages/web/dist",
        "assets/icons",
        "assets/photos",
        "assets/icons/cache",
        "docs",
        "docs/drafts",
        "target/debug/build",
        "config",
        "scripts",
        "tmp",
        ".git/objects",
    ];
    let stems = [
        "main",
        "lib",
        "mod",
        "parser",
        "matcher",
        "glob",
        "readme",
        "index",
        "style",
        "setup",
        "unit_test",
        "integration_test",
        "snapshot",
        "bundle",
        "cache",
        "bindings.generated",
    ];
    let exts = [
        "rs", "rs", "ts", "tsx", "js", "md", "png", "jpg", "svg", "toml", "lock", "yaml", "sh",
        "snap", "txt",
    ];

    (0..size)
        .map(|i| {
            let root = roots[i % roots.len()];
            let stem = stems[(i / roots.len()) % stems.len()];
            let ext = exts[(i / (roots.len() * stems.len())) % exts.len()];
            match (stem, ext, i % 11) {
                (_, "toml", 0) => format!("{root}/Cargo.toml"),
                (_, "md", 0) => format!("{root}/README.md"),
                _ => format!("{root}/level-{}/branch-{}/{stem}.{ext}", i % 17, i % 31),
            }
        })
        .collect()
}

fn typescript_corpus(size: usize) -> Vec<String> {
    let roots = [
        "foo",
        "foo/bar",
        "src",
        "src/features",
        "packages/web/src",
        "packages/api/src",
        "packages/web/node_modules/react",
        "packages/web/dist",
        "tests",
        "tmp",
    ];
    let stems = [
        "index",
        "app",
        "route",
        "model",
        "service",
        "component",
        "generated",
        "spec",
    ];
    let exts = ["ts", "tsx", "js", "d.ts", "map"];

    (0..size)
        .map(|i| {
            if i % 37 == 0 {
                return format!("foo/{}.ts", stems[i % stems.len()]);
            }
            let root = roots[i % roots.len()];
            let stem = stems[(i / roots.len()) % stems.len()];
            let ext = exts[(i / (roots.len() * stems.len())) % exts.len()];
            format!("{root}/level-{}/branch-{}/{stem}.{ext}", i % 13, i % 19)
        })
        .collect()
}

fn compile_glob_patterns() -> Vec<Pattern> {
    INCLUDE_PATTERNS
        .iter()
        .chain(EXCLUDE_PATTERNS)
        .map(|pattern| Pattern::new(pattern).unwrap())
        .collect()
}

fn compile_glob_list(patterns: &[&str]) -> Vec<Pattern> {
    patterns
        .iter()
        .map(|pattern| Pattern::new(pattern).unwrap())
        .collect()
}

fn compile_globset(patterns: &[&str]) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern).unwrap());
    }
    builder.build().unwrap()
}

fn compile_globsets() -> (GlobSet, GlobSet) {
    (
        compile_globset(INCLUDE_PATTERNS),
        compile_globset(EXCLUDE_PATTERNS),
    )
}

fn glob_regex(pattern: &str) -> String {
    Glob::new(pattern).unwrap().regex().to_owned()
}

fn pcre2_glob_regex(pattern: &str) -> String {
    glob_regex(pattern).replace("(?-u)", "")
}

fn compile_regex_set(patterns: &[&str]) -> RegexSet {
    let regexes = patterns
        .iter()
        .map(|pattern| glob_regex(pattern))
        .collect::<Vec<_>>();
    RegexSet::new(regexes).unwrap()
}

fn compile_regex_sets(patterns: (&[&str], &[&str])) -> (RegexSet, RegexSet) {
    (compile_regex_set(patterns.0), compile_regex_set(patterns.1))
}

fn compile_pcre2_jit_set(patterns: &[&str]) -> Vec<Pcre2Regex> {
    let mut builder = Pcre2RegexBuilder::new();
    builder.jit_if_available(true);
    patterns
        .iter()
        .map(|pattern| builder.build(&pcre2_glob_regex(pattern)).unwrap())
        .collect()
}

fn compile_pcre2_jit_sets(patterns: (&[&str], &[&str])) -> (Vec<Pcre2Regex>, Vec<Pcre2Regex>) {
    (
        compile_pcre2_jit_set(patterns.0),
        compile_pcre2_jit_set(patterns.1),
    )
}

fn count_fast_glob(
    include_patterns: &[&str],
    exclude_patterns: &[&str],
    paths: &[String],
) -> usize {
    paths
        .iter()
        .filter(|path| {
            include_patterns
                .iter()
                .any(|pattern| fast_glob::glob_match(pattern, path))
                && !exclude_patterns
                    .iter()
                    .any(|pattern| fast_glob::glob_match(pattern, path))
        })
        .count()
}

fn count_glob(
    include_patterns: &[Pattern],
    exclude_patterns: &[Pattern],
    paths: &[String],
) -> usize {
    paths
        .iter()
        .filter(|path| {
            let path = Path::new(path.as_str());
            include_patterns
                .iter()
                .any(|pattern| pattern.matches_path(path))
                && !exclude_patterns
                    .iter()
                    .any(|pattern| pattern.matches_path(path))
        })
        .count()
}

fn count_globset(include_set: &GlobSet, exclude_set: &GlobSet, paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| include_set.is_match(path.as_str()) && !exclude_set.is_match(path.as_str()))
        .count()
}

fn count_wax(include_any: &wax::Any<'_>, exclude_any: &wax::Any<'_>, paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| include_any.is_match(path.as_str()) && !exclude_any.is_match(path.as_str()))
        .count()
}

fn count_regex_set(include_set: &RegexSet, exclude_set: &RegexSet, paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| {
            include_set.is_match(path.as_bytes()) && !exclude_set.is_match(path.as_bytes())
        })
        .count()
}

fn count_pcre2_jit(
    include_set: &[Pcre2Regex],
    exclude_set: &[Pcre2Regex],
    paths: &[String],
) -> usize {
    paths
        .iter()
        .filter(|path| {
            let path = path.as_bytes();
            include_set
                .iter()
                .any(|regex| regex.is_match(path).unwrap())
                && !exclude_set
                    .iter()
                    .any(|regex| regex.is_match(path).unwrap())
        })
        .count()
}

fn bench_pattern_list(
    c: &mut Criterion,
    group_name: &str,
    include_patterns: &[&str],
    exclude_patterns: &[&str],
    make_corpus: fn(usize) -> Vec<String>,
    bench_glob_scan: bool,
    bench_wax: bool,
) {
    let mut group = c.benchmark_group(group_name);

    for size in [256, 4_096, 65_536] {
        let paths = make_corpus(size);
        let glob_patterns = bench_glob_scan.then(|| {
            (
                compile_glob_list(include_patterns),
                compile_glob_list(exclude_patterns),
            )
        });
        let globset_include = compile_globset(include_patterns);
        let globset_exclude = compile_globset(exclude_patterns);
        let regex_include = compile_regex_set(include_patterns);
        let regex_exclude = compile_regex_set(exclude_patterns);
        let pcre2_include = compile_pcre2_jit_set(include_patterns);
        let pcre2_exclude = compile_pcre2_jit_set(exclude_patterns);
        let wax_patterns = bench_wax.then(|| {
            (
                wax::any(include_patterns.iter().copied()).unwrap(),
                wax::any(exclude_patterns.iter().copied()).unwrap(),
            )
        });
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("fast-glob/scan", size),
            &paths,
            |b, paths| {
                b.iter(|| {
                    count_fast_glob(
                        black_box(include_patterns),
                        black_box(exclude_patterns),
                        black_box(paths),
                    )
                });
            },
        );

        if let Some((glob_include, glob_exclude)) = &glob_patterns {
            group.bench_with_input(BenchmarkId::new("glob/scan", size), &paths, |b, paths| {
                b.iter(|| {
                    count_glob(
                        black_box(glob_include),
                        black_box(glob_exclude),
                        black_box(paths),
                    )
                });
            });
        }

        group.bench_with_input(BenchmarkId::new("globset/set", size), &paths, |b, paths| {
            b.iter(|| {
                count_globset(
                    black_box(&globset_include),
                    black_box(&globset_exclude),
                    black_box(paths),
                )
            });
        });

        group.bench_with_input(BenchmarkId::new("regex/set", size), &paths, |b, paths| {
            b.iter(|| {
                count_regex_set(
                    black_box(&regex_include),
                    black_box(&regex_exclude),
                    black_box(paths),
                )
            });
        });

        group.bench_with_input(
            BenchmarkId::new("pcre2/jit-scan", size),
            &paths,
            |b, paths| {
                b.iter(|| {
                    count_pcre2_jit(
                        black_box(&pcre2_include),
                        black_box(&pcre2_exclude),
                        black_box(paths),
                    )
                });
            },
        );

        if let Some((wax_include, wax_exclude)) = &wax_patterns {
            group.bench_with_input(BenchmarkId::new("wax/any", size), &paths, |b, paths| {
                b.iter(|| {
                    count_wax(
                        black_box(wax_include),
                        black_box(wax_exclude),
                        black_box(paths),
                    )
                });
            });
        }
    }

    group.finish();
}

fn bench_multi_pattern(c: &mut Criterion) {
    bench_pattern_list(
        c,
        "ignore_list_match",
        INCLUDE_PATTERNS,
        EXCLUDE_PATTERNS,
        corpus,
        true,
        true,
    );
}

fn bench_typescript_negative(c: &mut Criterion) {
    bench_pattern_list(
        c,
        "typescript_negative_match",
        TYPESCRIPT_INCLUDE_PATTERNS,
        TYPESCRIPT_EXCLUDE_PATTERNS,
        typescript_corpus,
        false,
        false,
    );
}

fn bench_compile(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_patterns");
    group.throughput(Throughput::Elements(
        (INCLUDE_PATTERNS.len() + EXCLUDE_PATTERNS.len()) as u64,
    ));

    group.bench_function("glob", |b| {
        b.iter(|| compile_glob_patterns());
    });

    group.bench_function("globset", |b| {
        b.iter(|| compile_globsets());
    });

    group.bench_function("regex_set", |b| {
        b.iter(|| {
            compile_regex_sets(black_box((INCLUDE_PATTERNS, EXCLUDE_PATTERNS)));
        });
    });

    group.bench_function("pcre2_jit_scan", |b| {
        b.iter(|| {
            compile_pcre2_jit_sets(black_box((INCLUDE_PATTERNS, EXCLUDE_PATTERNS)));
        });
    });

    group.bench_function("wax_any", |b| {
        b.iter(|| {
            (
                wax::any(black_box(INCLUDE_PATTERNS.iter().copied())).unwrap(),
                wax::any(black_box(EXCLUDE_PATTERNS.iter().copied())).unwrap(),
            )
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_multi_pattern,
    bench_typescript_negative,
    bench_compile
);
criterion_main!(benches);
