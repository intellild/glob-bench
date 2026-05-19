use std::hint::black_box;
use std::path::Path;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use glob::Pattern;
use globset::{Glob, GlobSet, GlobSetBuilder};
use wax::Program;

const PATTERNS: &[&str] = &[
    "src/**/*.rs",
    "src/**/mod.rs",
    "tests/**/*_test.rs",
    "benches/**/*.rs",
    "examples/**/*.rs",
    "crates/*/src/**/*.rs",
    "packages/*/src/**/*.ts",
    "assets/**/*.png",
    "assets/**/*.jpg",
    "docs/**/*.md",
    "**/Cargo.toml",
    "**/README.md",
    "**/*.lock",
    "scripts/*.sh",
    "config/*.yaml",
    "target/**",
];

fn corpus(size: usize) -> Vec<String> {
    let roots = [
        "src",
        "tests",
        "benches",
        "examples",
        "crates/core/src",
        "crates/cli/src",
        "packages/web/src",
        "assets/icons",
        "assets/photos",
        "docs",
        "target/debug/build",
        "config",
        "scripts",
        "tmp",
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
    ];
    let exts = [
        "rs", "rs", "ts", "js", "md", "png", "jpg", "toml", "lock", "yaml", "sh", "txt",
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

fn compile_glob_patterns() -> Vec<Pattern> {
    PATTERNS
        .iter()
        .map(|pattern| Pattern::new(pattern).unwrap())
        .collect()
}

fn compile_globset() -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for pattern in PATTERNS {
        builder.add(Glob::new(pattern).unwrap());
    }
    builder.build().unwrap()
}

fn count_fast_glob(paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| {
            PATTERNS
                .iter()
                .any(|pattern| fast_glob::glob_match(pattern, path))
        })
        .count()
}

fn count_glob(patterns: &[Pattern], paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| {
            let path = Path::new(path.as_str());
            patterns.iter().any(|pattern| pattern.matches_path(path))
        })
        .count()
}

fn count_globset(globset: &GlobSet, paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| globset.is_match(path.as_str()))
        .count()
}

fn count_wax(any: &wax::Any<'_>, paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| any.is_match(path.as_str()))
        .count()
}

fn bench_multi_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_pattern_match");

    for size in [256, 4_096, 65_536] {
        let paths = corpus(size);
        let glob_patterns = compile_glob_patterns();
        let globset = compile_globset();
        let wax_any = wax::any(PATTERNS.iter().copied()).unwrap();
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("fast-glob/scan", size),
            &paths,
            |b, paths| {
                b.iter(|| count_fast_glob(black_box(paths)));
            },
        );

        group.bench_with_input(BenchmarkId::new("glob/scan", size), &paths, |b, paths| {
            b.iter(|| count_glob(black_box(&glob_patterns), black_box(paths)));
        });

        group.bench_with_input(BenchmarkId::new("globset/set", size), &paths, |b, paths| {
            b.iter(|| count_globset(black_box(&globset), black_box(paths)));
        });

        group.bench_with_input(BenchmarkId::new("wax/any", size), &paths, |b, paths| {
            b.iter(|| count_wax(black_box(&wax_any), black_box(paths)));
        });
    }

    group.finish();
}

fn bench_compile(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_patterns");
    group.throughput(Throughput::Elements(PATTERNS.len() as u64));

    group.bench_function("glob", |b| {
        b.iter(|| compile_glob_patterns());
    });

    group.bench_function("globset", |b| {
        b.iter(|| compile_globset());
    });

    group.bench_function("wax_any", |b| {
        b.iter(|| wax::any(black_box(PATTERNS.iter().copied())).unwrap());
    });

    group.finish();
}

criterion_group!(benches, bench_multi_pattern, bench_compile);
criterion_main!(benches);
