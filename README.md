# mic

[![CI](https://github.com/qryxip/mic/workflows/CI/badge.svg)](https://github.com/qryxip/mic/actions?workflow=CI)
[![Rust 2018 1.42.0+](https://img.shields.io/badge/rust%202018-1.42.0+-lightgray.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/mic.svg)](https://crates.io/crates/mic)
[![Crates.io](https://img.shields.io/crates/l/mic.svg)](https://crates.io/crates/mic)

Facilitates answering to competitive programming problems.

This crate is intended to be used with [cargo-equip](https://github.com/qryxip/cargo-equip), which is a tool to bundle code into single `.rs` file.

## Usage

See [the documentation on Docs.rs](https://docs.rs/mic).

## Examples

```rust
use mic::{answer, solve};
```

```rust
#[answer]
fn main() -> _ {
    1
}
// 1 → println!("{}", 1)
```

```rust
#[answer(yn("Yes", "No"))]
fn main() -> _ {
    true
}
// true → "Yes"
//      → println!("{}", "Yes")
```

```rust
#[answer(tuple(" "))]
fn main() -> _ {
    (42, "foo")
}
// (42, "foo") → "42 foo".to_owned()
//             → println!("{}", "42 foo".to_owned())
```

```rust
#[answer(join("\n"))]
fn main() -> _ {
    1..=3
}
// 1..=3 → "1\n2\n3".to_owned()
//       → println!("{}", "1\n2\n3".to_owned())
```

```rust
#[answer(matrix(" "))]
fn main() -> _ {
    vec![vec![1, 2], vec![3, 4]]
}
// vec![vec![1, 2], vec![3, 4]] → "1 2\n3 4".to_owned()
//                              → println!("{}", "1 2\n3 4".to_owned())
```

```rust
#[answer(join(" "), map(add(1)))]
fn main() -> _ {
    vec![0, 2, 4] // 0-based graph node indices
}
// vec![0, 2, 4] → { impl Iterator } ([1, 3, 5])
//               → "1 3 5".to_owned()
//               → println!("{}", "1 3 5".to_owned())
```

```
#[solve(join(" "))]
fn solve() -> _ {
    1..=3
}
// 1..=3 → "1 2 3".to_owned()

assert_eq!("1 2 3", solve());
```

## License

Licensed under [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/).
