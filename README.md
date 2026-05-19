# glob_benchs

Rust benchmarks for comparing glob matching crates:

- `fast-glob`
- `glob`
- `globset`
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

Benchmark groups:

- `ignore_list_match`: prebuilds reusable matchers where the crate supports it, then counts how many
  generated paths match an include pattern and do not match an exclude pattern.
- `compile_patterns`: measures the cost of compiling/building the pattern set.

Notes:

- `globset` is measured with compiled include and exclude `GlobSet`s.
- `wax` is measured with compiled include and exclude `wax::any(...)` combinators.
- `fast-glob` and `glob` are measured by scanning the pattern list, because this benchmark only uses
  their public APIs for individual pattern matching.
