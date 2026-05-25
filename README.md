# glob_benchs

Rust benchmarks for comparing glob matching crates:

- `fast-glob`
- `glob`
- `globset`
- `regex` using `globset::Glob::regex()` glob-to-regexp output
- `pcre2` with JIT requested, also using `globset::Glob::regex()` output
- `wax`

The main benchmark focuses on matching many paths against an ignore-style glob list: include
patterns first, then exclude patterns. It keeps the input corpus deterministic and in memory so the
measurement is about matcher performance, not filesystem traversal.

Run the full benchmark:

```sh
cargo bench --bench multi_pattern
```

For a quicker smoke run:

```sh
cargo bench --bench multi_pattern -- --sample-size 10
```

Measure stripped release binary size for each implementation probe:

```sh
cargo build --release --manifest-path size-probes/Cargo.toml --bin regex_set --features globset,regex
stat -c '%n %s' size-probes/target/release/regex_set
```

Benchmark groups:

- `ignore_list_match`: prebuilds reusable matchers where the crate supports it, then counts how many
  generated paths match an include pattern and do not match an exclude pattern.
- `typescript_negative_match`: benchmarks the explicit `["**/**.ts", "!foo/*.ts"]` negative-pattern
  scenario. `glob` and `wax` are skipped in this group because they reject `**/**.ts`.
- `compile_patterns`: measures the cost of compiling/building the pattern set.

Notes:

- `globset` is measured with compiled include and exclude `GlobSet`s.
- `regex` is measured with include and exclude `regex::bytes::RegexSet`s generated from
  `globset::Glob::regex()`.
- `pcre2` is measured with include and exclude lists of compiled regexes. It uses
  `jit_if_available(true)` and scans the regex list.
- `wax` is measured with compiled include and exclude `wax::any(...)` combinators.
- `fast-glob` and `glob` are measured by scanning the pattern list, because this benchmark only uses
  their public APIs for individual pattern matching.
- `size-probes` contains minimal binaries for comparing stripped release binary size per
  implementation combination.
