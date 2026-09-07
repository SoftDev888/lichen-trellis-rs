fn divisor(mut left: i64, mut right: i64) -> i64 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left.abs()
}

fn main() {
    println!("{}", divisor(1071, 462));
}
