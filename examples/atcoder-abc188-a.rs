use mic::answer;
use proconio::input;
use std::cmp;

#[answer(yn("Yes", "No"))]
fn main() -> _ {
    input!(x: u32, y: u32);
    cmp::min(x, y) + 3 > cmp::max(x, y)
}
