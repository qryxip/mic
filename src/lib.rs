#![forbid(unsafe_code)]
#![warn(
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements
)]
//! Facilitates answering to competitive programming problems.
//!
//! This crate is intended to be used with [cargo-equip], which is a tool to bundle code into single `.rs` file.
//!
//! # Examples
//!
//! ```no_run
//! use mic::{answer, solve};
//! ```
//!
//! ```no_run
//! # use mic::answer;
//! #[answer]
//! fn main() -> _ {
//!     1
//! }
//! // 1 → println!("{}", 1)
//! ```
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(yn("Yes", "No"))]
//! fn main() -> _ {
//!     true
//! }
//! // true → "Yes"
//! //      → println!("{}", "Yes")
//! ```
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(tuple(" "))]
//! fn main() -> _ {
//!     (42, "foo")
//! }
//! // (42, "foo") → "42 foo".to_owned()
//! //             → println!("{}", "42 foo".to_owned())
//! ```
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(join("\n"))]
//! fn main() -> _ {
//!     1..=3
//! }
//! // 1..=3 → "1\n2\n3".to_owned()
//! //       → println!("{}", "1\n2\n3".to_owned())
//! ```
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(matrix(" "))]
//! fn main() -> _ {
//!     vec![vec![1, 2], vec![3, 4]]
//! }
//! // vec![vec![1, 2], vec![3, 4]] → "1 2\n3 4".to_owned()
//! //                              → println!("{}", "1 2\n3 4".to_owned())
//! ```
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(join(" "), map(add(1)))]
//! fn main() -> _ {
//!     vec![0, 2, 4] // 0-based graph node indices
//! }
//! // vec![0, 2, 4] → { impl Iterator } ([1, 3, 5])
//! //               → "1 3 5".to_owned()
//! //               → println!("{}", "1 3 5".to_owned())
//! ```
//!
//! ```
//! # use mic::solve;
//! # fn main() {
//! #[solve(join(" "))]
//! fn solve() -> _ {
//!     1..=3
//! }
//! // 1..=3 → "1 2 3".to_owned()
//!
//! assert_eq!("1 2 3", solve());
//! # }
//! ```
//!
//! # Quickstart
//!
//! ```no_run
//! use mic::answer;
//!
//! #[answer(std::convert::identity)]
//! fn main() -> _ {
//!     42
//! }
//! ```
//!
//! This `main` function is converted into:
//!
//! ```no_run
//! # use std::convert;
//! fn main() {
//!    #[allow(unused_imports)]
//!    use ::mic::__YouCannotRecurseIfTheOutputTypeIsInferred as main;
//!
//!    let __mic_ans = (move || -> _ {
//!        42
//!    })();
//!    let __mic_ans = {
//!        #[allow(unused_imports)]
//!        use ::mic::filters::*;
//!
//!        std::convert::identity(__mic_ans)
//!    };
//!    ::std::println!("{}", __mic_ans);
//! }
//! ```
//!
//! `#[answer]` takes 0 or more functions as arguments.
//! If multiple functions are given, they are composed in right associative.
//!
//! ```no_run
//! # use mic::answer;
//! #[answer]
//! fn main() -> _ {
//!     1
//! }
//! // 1 → println!("{}", 1)
//! ```
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(|ans| ans - 1, |ans| ans * 2, |ans| ans + 3)]
//! fn main() -> _ {
//!     1
//! }
//! // 1 → 4 (+ 3)
//! //   → 8 (* 2)
//! //   → 7 (- 1)
//! //   → println!("{}", 7)
//! ```
//!
//! # Filters
//!
//! This crate provides some utility functions under the [`mic::filters`] module.
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(mic::filters::join("\n"))]
//! fn main() -> _ {
//!     vec![1, 2, 3]
//! }
//! // vec![1, 2, 3] → "1\n2\n3".to_owned()
//! //               → println!("{}", "1\n2\n3".to_owned())
//! ```
//!
//! `mic::filters::` prefixes can be omitted since the functions are `use`d inside.
//!
//! ```no_run
//! # use mic::answer;
//! #[answer(join("\n"))]
//! # fn main() -> _ {
//! #     vec![1, 2, 3]
//! # }
//! ```
//!
//! # `#[solve]`
//!
//! [<code>#\[solve\]</code>] wraps the output in `ToString::to_string` instead of `println!`.
//!
//! ```no_run
//! use mic::solve;
//! use proconio::{input, source::once::OnceSource};
//! use std::io::{self, Read as _};
//!
//! fn main() {
//!     let mut input = "".to_owned();
//!     io::stdin().read_to_string(&mut input).unwrap();
//!     println!("{}", solve(&input));
//! }
//!
//! #[solve(join(" "))]
//! fn solve(input: &str) -> _ {
//!     input! {
//!         from OnceSource::from(input),
//!         mut xs: [u32],
//!     }
//!     xs.reverse();
//!     xs
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::solve;
//!
//!     #[test]
//!     fn test() {
//!         assert_eq!("3 2 1", solve("3\n1 2 3\n"));
//!     }
//! }
//! ```
//!
//! [cargo-equip]: https://github.com/qryxip/cargo-equip
//! [`mic::filters`]: ./filters/index.html
//! [<code>#\[solve\]</code>]: ./attr.solve.html

pub mod filters;

mod extern_crates {
    pub(super) extern crate mic_impl;
}

#[doc(inline)]
pub use self::extern_crates::mic_impl::*;

#[doc(hidden)]
pub struct __YouCannotRecurseIfTheOutputTypeIsInferred;
