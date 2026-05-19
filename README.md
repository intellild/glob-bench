# glob_benchs

Rust benchmarks for comparing glob matching crates:

- `fast-glob`
- `glob`
- `globset`
- `wax`

The main benchmark focuses on matching many paths against multiple patterns. It keeps the input
corpus deterministic and in memory so the measurement is about matcher performance, not filesystem
traversal.

Run the full benchmark:

```sh
cargo bench --bench multi_pattern
```

For a quicker smoke run:

```sh
cargo bench --bench multi_pattern -- --sample-size 10
```

Benchmark groups:

- `multi_pattern_match`: prebuilds reusable matchers where the crate supports it, then counts how
  many generated paths match any pattern.
- `compile_patterns`: measures the cost of compiling/building the pattern set.

Notes:

- `globset` is measured with a compiled `GlobSet`.
- `wax` is measured with a compiled `wax::any(...)` combinator.
- `fast-glob` and `glob` are measured by scanning the pattern list, because this benchmark only uses
  their public APIs for individual pattern matching.
