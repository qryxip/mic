//! The functions for conversion.

use std::{convert, fmt::Display, iter::Map, ops::Add};

/// Behaves like `|f, x| f(x)`.
///
/// ```
/// use mic::filters::apply;
/// assert_eq!(2, apply(|x| x + 1, 1));
/// ```
pub fn apply<F: FnOnce(X) -> Y, X, Y>(f: F, x: X) -> Y {
    f(x)
}

/// Behaves like `|f, g| move |x| f(g(x))`.
///
/// ```
/// use mic::filters::dot;
/// assert_eq!(4, dot(|x| 2 * x, |x| x + 1)(1));
/// ```
pub fn dot<F: FnOnce(Y) -> Z, G: FnOnce(X) -> Y, X, Y, Z>(f: F, g: G) -> impl FnOnce(X) -> Z {
    move |x| f(g(x))
}

/// Behaves like `|f| move |x| f(x)`.
///
/// ```
/// use mic::filters::map;
/// assert_eq!(
///     [2, 3, 4],
///     *map(|x| x + 1)(vec![1, 2, 3]).collect::<Vec<_>>(),
/// );
/// ```
pub fn map<F: FnMut(X) -> Y, I: IntoIterator<Item = X>, X, Y>(
    f: F,
) -> impl FnOnce(I) -> Map<I::IntoIter, F> {
    move |xs| xs.into_iter().map(f)
}

/// Behaves like `|sep| move |xs| xs.into_iter().join(sep)` (under [itertools](https://docs.rs/crate/itertools)).
///
/// ```
/// use mic::filters::join;
/// assert_eq!("1 2 3", join(" ")(vec![1, 2, 3]));
/// assert_eq!("1\n2\n3", join("\n")(vec![1, 2, 3]));
/// ```
pub fn join<I: IntoIterator<Item = T>, T: Display>(sep: &'static str) -> impl FnMut(I) -> String {
    move |xs| {
        xs.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

/// Behaves like `|sep| dot(join("\n"), map(join(sep)))`.
///
/// ```
/// use mic::filters::matrix;
/// assert_eq!("1 2\n3 4", matrix(" ")(vec![vec![1, 2], vec![3, 4]]));
/// ```
pub fn matrix<I: IntoIterator<Item = J>, J: IntoIterator<Item = T>, T: Display>(
    sep: &'static str,
) -> impl FnMut(I) -> String {
    move |xs| join("\n")(map(join(sep))(xs))
}

/// Behaves like `|lhs| move |rhs| lhs + rhs`.
///
/// ```
/// use mic::filters::add;
/// assert_eq!(11, add(1)(10));
/// ```
pub fn add<L: Copy + Add<R, Output = O>, R, O>(lhs: L) -> impl FnMut(R) -> O {
    move |rhs| lhs + rhs
}

/// Behaves like `|yes, no| move |p| if p { yes } else { no }`.
///
/// ```
/// use mic::filters::yn;
/// assert_eq!("Yes", yn("Yes", "No")(true));
/// assert_eq!("No", yn("Yes", "No")(false));
/// ```
pub fn yn(yes: &'static str, no: &'static str) -> impl FnMut(bool) -> &'static str {
    move |p| if p { yes } else { no }
}

pub fn or<A: Display, B: Display>(or: B) -> impl FnMut(Option<A>) -> String {
    move |x| x.map(|x| x.to_string()).unwrap_or_else(|| or.to_string())
}

/// Behaves like `|sep| move |(a, b, …): T| format!(…)`.
///
/// ```
/// use mic::filters::tuple;
/// assert_eq!("1", tuple(" ")((1,)));
/// assert_eq!("1 a", tuple(" ")((1, 'a')));
/// assert_eq!("1 a abc", tuple(" ")((1, 'a', "abc")));
/// assert_eq!("1 a abc 1.1", tuple(" ")((1, 'a', "abc", &&&&&1.1)));
/// ```
pub fn tuple<T: DisplayTuple>(sep: &'static str) -> impl FnMut(T) -> String {
    move |tuple| tuple.format(sep)
}

/// Implemented for tuple of size up to 4 that consist of [`Display`] elements.
///
/// [`Display`]: https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html
pub trait DisplayTuple {
    fn format(&self, sep: &str) -> String;
}

macro_rules! impl_display_tuple(($(impl<$($type_param:ident : _),*> _ for $self_ty:ty { $fn:expr };)*) => {
    $(
        impl<$($type_param: Display),*> DisplayTuple for $self_ty {
            fn format(&self, sep: &str) -> String {
                convert::identity::<fn(&_, _) -> _>($fn)(self, sep)
            }
        }
    )*
});

impl_display_tuple! {
    impl<A: _>                   _ for (A,)         { |(a,)        , _  | format!("{}", a)                             };
    impl<A: _, B: _>             _ for (A, B)       { |(a, b)      , sep| format!("{}{}{}", a, sep, b)                 };
    impl<A: _, B: _, C: _>       _ for (A, B, C)    { |(a, b, c)   , sep| format!("{}{}{}{1}{}", a, sep, b, c)         };
    impl<A: _, B: _, C: _, D: _> _ for (A, B, C, D) { |(a, b, c, d), sep| format!("{}{}{}{1}{}{1}{}", a, sep, b, c, d) };
}
