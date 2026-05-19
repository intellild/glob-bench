# Glob Benchmark Results

Run time: 2026-05-19T10:13:51Z

Command:

```sh
cargo bench --bench multi_pattern
```

Environment:

- Commit: `64da485`
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- Criterion samples: 100
- Plot backend: plotters (`gnuplot` was not installed)

## Ignore List Match

Each benchmark counts how many generated paths match an include pattern and do not match an exclude
pattern. The benchmark uses 16 include patterns and 10 exclude patterns. `globset` and `wax` use
prebuilt include and exclude matchers; `fast-glob` and `glob` scan the pattern lists.

| Paths | Crate / Mode | Time | Throughput |
| ---: | --- | ---: | ---: |
| 256 | `fast-glob/scan` | 87.197 us | 2.9359 Melem/s |
| 256 | `glob/scan` | 119.85 us | 2.1361 Melem/s |
| 256 | `globset/set` | 25.378 us | 10.088 Melem/s |
| 256 | `wax/any` | 9.2619 us | 27.640 Melem/s |
| 4,096 | `fast-glob/scan` | 1.1198 ms | 3.6579 Melem/s |
| 4,096 | `glob/scan` | 1.7293 ms | 2.3685 Melem/s |
| 4,096 | `globset/set` | 200.51 us | 20.427 Melem/s |
| 4,096 | `wax/any` | 137.71 us | 29.743 Melem/s |
| 65,536 | `fast-glob/scan` | 18.075 ms | 3.6257 Melem/s |
| 65,536 | `glob/scan` | 28.491 ms | 2.3002 Melem/s |
| 65,536 | `globset/set` | 3.1156 ms | 21.035 Melem/s |
| 65,536 | `wax/any` | 2.2117 ms | 29.631 Melem/s |

## Pattern Compilation

This group measures building/compiling the 16 include patterns and 10 exclude patterns.

| Crate / Mode | Time | Throughput |
| --- | ---: | ---: |
| `glob` | 5.0261 us | 5.1730 Melem/s |
| `globset` | 150.85 us | 172.36 Kelem/s |
| `wax_any` | 313.47 us | 82.942 Kelem/s |

## Notes

- `wax/any` was fastest in this in-memory ignore-list matching workload.
- `globset/set` was the next fastest matcher and substantially faster than scanning individual
  patterns.
- `glob` had the cheapest pattern compilation cost, but this benchmark uses it as a pattern-list
  scan during matching.
- Criterion reported regressions against the previous baseline for the compilation group, but this
  run uses a different benchmark shape: 26 total patterns split into include and exclude lists.
- Criterion HTML reports are available under `target/criterion/report/index.html`.
