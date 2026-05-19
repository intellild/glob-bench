use std::hint::black_box;
use std::path::Path;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use glob::Pattern;
use globset::{Glob, GlobSet, GlobSetBuilder};
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

fn count_fast_glob(paths: &[String]) -> usize {
    paths
        .iter()
        .filter(|path| {
            INCLUDE_PATTERNS
                .iter()
                .any(|pattern| fast_glob::glob_match(pattern, path))
                && !EXCLUDE_PATTERNS
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

fn bench_multi_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("ignore_list_match");

    for size in [256, 4_096, 65_536] {
        let paths = corpus(size);
        let glob_include = compile_glob_list(INCLUDE_PATTERNS);
        let glob_exclude = compile_glob_list(EXCLUDE_PATTERNS);
        let (globset_include, globset_exclude) = compile_globsets();
        let wax_include = wax::any(INCLUDE_PATTERNS.iter().copied()).unwrap();
        let wax_exclude = wax::any(EXCLUDE_PATTERNS.iter().copied()).unwrap();
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("fast-glob/scan", size),
            &paths,
            |b, paths| {
                b.iter(|| count_fast_glob(black_box(paths)));
            },
        );

        group.bench_with_input(BenchmarkId::new("glob/scan", size), &paths, |b, paths| {
            b.iter(|| {
                count_glob(
                    black_box(&glob_include),
                    black_box(&glob_exclude),
                    black_box(paths),
                )
            });
        });

        group.bench_with_input(BenchmarkId::new("globset/set", size), &paths, |b, paths| {
            b.iter(|| {
                count_globset(
                    black_box(&globset_include),
                    black_box(&globset_exclude),
                    black_box(paths),
                )
            });
        });

        group.bench_with_input(BenchmarkId::new("wax/any", size), &paths, |b, paths| {
            b.iter(|| {
                count_wax(
                    black_box(&wax_include),
                    black_box(&wax_exclude),
                    black_box(paths),
                )
            });
        });
    }

    group.finish();
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

criterion_group!(benches, bench_multi_pattern, bench_compile);
criterion_main!(benches);
