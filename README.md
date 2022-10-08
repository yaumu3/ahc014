# AHC014

[AHC014](https://atcoder.jp/contests/ahc014) + parallel tester

## Test script

Parallel tests can be run with `runner.py`. The number of parallelism is limited to `cpu_count() - 4` so that other apps do not to affect the test results.

### Prerequisites

- [ ] Python 3.9 or later
- [ ] Rust 1.42 or later
- [ ] Installed local tester (You can download it from the [Problem statement](https://atcoder.jp/contests/ahc014/tasks/ahc014_a) page.)
- [ ] Generated test cases with the local tester.

### Usage

Please refer to the help by `python runner.py --help`.

### Example

One-liner to build + run tests by `runner.py` assuming local tester resides in `../ahc014-tools` relative to this document's directory.

```bash
cargo build --release && python runner.py target/release/ahc014 ../ahc014-tools ../ahc014-tools/in ../ahc014-tools/out
```
