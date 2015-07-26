/// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn euler1(bound: u32) -> u32 {
    (1..bound)
    .filter(|n| n % 3 == 0 || n % 5 == 0)
    .fold(0, |s, n| s + n)
}
