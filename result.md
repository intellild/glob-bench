# Glob Benchmark Results

Run time: 2026-05-22T08:13:07Z

Command:

```sh
cargo bench --bench multi_pattern
```

Environment:

- Commit: `64da485` plus local benchmark changes
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- Criterion samples: 100
- Plot backend: plotters (`gnuplot` was not installed)
- PCRE2: 10.47; `pcre2test -C jit` returned `1`

## Ignore List Match

This group counts generated repository-like paths that match any include pattern and do not match
any exclude pattern. `globset`, `regex`, and `wax` use prebuilt include/exclude matchers. `fast-glob`,
`glob`, and `pcre2` scan their include/exclude pattern lists.

| Paths | Crate / Mode | Time | Throughput |
| ---: | --- | ---: | ---: |
| 256 | `fast-glob/scan` | 224.03 us | 1.1427 Melem/s |
| 256 | `glob/scan` | 312.28 us | 819.78 Kelem/s |
| 256 | `globset/set` | 66.067 us | 3.8748 Melem/s |
| 256 | `regex/set` | 35.044 us | 7.3051 Melem/s |
| 256 | `pcre2/jit-scan` | 211.32 us | 1.2114 Melem/s |
| 256 | `wax/any` | 24.770 us | 10.335 Melem/s |
| 4,096 | `fast-glob/scan` | 2.7710 ms | 1.4782 Melem/s |
| 4,096 | `glob/scan` | 4.6024 ms | 889.96 Kelem/s |
| 4,096 | `globset/set` | 560.17 us | 7.3121 Melem/s |
| 4,096 | `regex/set` | 493.47 us | 8.3004 Melem/s |
| 4,096 | `pcre2/jit-scan` | 3.9611 ms | 1.0341 Melem/s |
| 4,096 | `wax/any` | 360.30 us | 11.368 Melem/s |
| 65,536 | `fast-glob/scan` | 44.109 ms | 1.4858 Melem/s |
| 65,536 | `glob/scan` | 72.476 ms | 904.25 Kelem/s |
| 65,536 | `globset/set` | 8.6285 ms | 7.5953 Melem/s |
| 65,536 | `regex/set` | 7.8370 ms | 8.3624 Melem/s |
| 65,536 | `pcre2/jit-scan` | 62.964 ms | 1.0409 Melem/s |
| 65,536 | `wax/any` | 6.0291 ms | 10.870 Melem/s |

## TypeScript Negative Match

This group uses the explicit pattern list `["**/**.ts", "!foo/*.ts"]`. `glob` and `wax` are skipped
for this group because they reject `**/**.ts` as an invalid recursive wildcard placement.

| Paths | Crate / Mode | Time | Throughput |
| ---: | --- | ---: | ---: |
| 256 | `fast-glob/scan` | 89.221 us | 2.8693 Melem/s |
| 256 | `globset/set` | 36.260 us | 7.0600 Melem/s |
| 256 | `regex/set` | 30.285 us | 8.4531 Melem/s |
| 256 | `pcre2/jit-scan` | 306.29 us | 835.81 Kelem/s |
| 4,096 | `fast-glob/scan` | 1.4229 ms | 2.8787 Melem/s |
| 4,096 | `globset/set` | 591.22 us | 6.9280 Melem/s |
| 4,096 | `regex/set` | 948.26 us | 4.3195 Melem/s |
| 4,096 | `pcre2/jit-scan` | 9.5770 ms | 427.69 Kelem/s |
| 65,536 | `fast-glob/scan` | 44.561 ms | 1.4707 Melem/s |
| 65,536 | `globset/set` | 15.596 ms | 4.2021 Melem/s |
| 65,536 | `regex/set` | 10.751 ms | 6.0960 Melem/s |
| 65,536 | `pcre2/jit-scan` | 135.09 ms | 485.14 Kelem/s |

## Pattern Compilation

This group measures building/compiling the 26-pattern ignore list.

| Crate / Mode | Time | Throughput |
| --- | ---: | ---: |
| `glob` | 10.448 us | 2.4885 Melem/s |
| `globset` | 460.99 us | 56.401 Kelem/s |
| `regex_set` | 347.01 us | 74.926 Kelem/s |
| `pcre2_jit_scan` | 248.64 us | 104.57 Kelem/s |
| `wax_any` | 883.19 us | 29.439 Kelem/s |

## Binary Size

This table measures stripped release binaries from `size-probes`, where each probe binary links only
the dependencies needed for that implementation combination. Command shape:

```sh
cargo build --release --offline --manifest-path size-probes/Cargo.toml --bin <bin> --features <features>
stat -c '%n %s' size-probes/target/release/<bin>
```

| Crate / Mode | Probe bin | Features | Size |
| --- | --- | --- | ---: |
| `fast-glob/scan` | `fast_glob` | `fast-glob` | 349,280 B / 341.1 KiB |
| `glob/scan` | `glob_scan` | `glob` | 367,056 B / 358.5 KiB |
| `globset/set` | `globset_set` | `globset` | 1,487,928 B / 1.42 MiB |
| `regex/set` | `regex_set` | `globset,regex` | 2,129,624 B / 2.03 MiB |
| `pcre2/jit-scan` | `pcre2_jit_scan` | `globset,pcre2` | 1,079,000 B / 1.03 MiB |
| `wax/any` | `wax_any` | `wax` | 1,906,848 B / 1.82 MiB |

## Notes

- `regex/set` uses `globset::Glob::regex()` as the glob-to-regexp implementation and compiles the
  output into `regex::bytes::RegexSet`.
- `pcre2/jit-scan` uses the same glob-to-regexp output, removes the Rust-regex-specific `(?-u)`
  marker for PCRE2 compatibility, requests JIT with `jit_if_available(true)`, and scans regexes one
  by one.
- In the larger ignore-list workload, `wax/any` remains fastest, followed by `regex/set` and
  `globset/set`.
- In the explicit TypeScript negative-pattern workload, `regex/set` is fastest at 256 and 65,536
  paths, while `globset/set` wins at 4,096 paths in this run.
- Criterion HTML reports are available under `target/criterion/report/index.html`.
