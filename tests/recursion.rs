use mic::{answer, solve};
use std::sync::atomic::{self, AtomicBool};

#[test]
fn answer() {
    let _: () = main();

    #[answer]
    fn main() -> &'static str {
        return if VISITED.swap(true, atomic::Ordering::SeqCst) {
            ""
        } else {
            main()
        };
        static VISITED: AtomicBool = AtomicBool::new(false);
    }
}

#[test]
fn solve() {
    assert_eq!("", solve(false));

    #[solve]
    fn solve(visited: bool) -> &'static str {
        if visited {
            ""
        } else {
            solve(true)
        }
    }
}
