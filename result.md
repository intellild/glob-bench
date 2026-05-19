# Glob Benchmark Results

Run time: 2026-05-19T09:46:43Z

Command:

```sh
cargo bench --bench multi_pattern
```

Environment:

- Commit: `ba79623`
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- Criterion samples: 100
- Plot backend: plotters (`gnuplot` was not installed)

## Multi Pattern Match

Each benchmark counts how many generated paths match any of 16 glob patterns. `globset` and `wax`
use prebuilt multi-pattern matchers; `fast-glob` and `glob` scan the pattern list.

| Paths | Crate / Mode | Time | Throughput |
| ---: | --- | ---: | ---: |
| 256 | `fast-glob/scan` | 137.22 us | 1.8657 Melem/s |
| 256 | `glob/scan` | 207.16 us | 1.2358 Melem/s |
| 256 | `globset/set` | 60.512 us | 4.2305 Melem/s |
| 256 | `wax/any` | 18.349 us | 13.952 Melem/s |
| 4,096 | `fast-glob/scan` | 2.8195 ms | 1.4527 Melem/s |
| 4,096 | `glob/scan` | 4.2491 ms | 963.97 Kelem/s |
| 4,096 | `globset/set` | 677.35 us | 6.0471 Melem/s |
| 4,096 | `wax/any` | 307.71 us | 13.311 Melem/s |
| 65,536 | `fast-glob/scan` | 45.703 ms | 1.4340 Melem/s |
| 65,536 | `glob/scan` | 69.773 ms | 939.27 Kelem/s |
| 65,536 | `globset/set` | 10.119 ms | 6.4764 Melem/s |
| 65,536 | `wax/any` | 5.0659 ms | 12.937 Melem/s |

## Pattern Compilation

This group measures building/compiling the 16-pattern set.

| Crate / Mode | Time | Throughput |
| --- | ---: | ---: |
| `glob` | 5.8671 us | 2.7270 Melem/s |
| `globset` | 263.12 us | 60.808 Kelem/s |
| `wax_any` | 504.71 us | 31.702 Kelem/s |

## Notes

- `wax/any` was fastest in this in-memory multi-pattern matching workload.
- `globset/set` was the next fastest matcher and substantially faster than scanning individual
  patterns.
- `glob` had the cheapest pattern compilation cost, but this benchmark uses it as a pattern-list
  scan during matching.
- Criterion HTML reports are available under `target/criterion/report/index.html`.
