/// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn euler1(bound: u32) -> u32 {
    (1..bound)
    .filter(|&n| n % 3 == 0 || n % 5 == 0)
    .fold(0, |s, n| s + n)
}


/// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
/// find the sum of the even-valued terms.
pub fn euler2_imperative(bound: u32) -> u32 {
    let mut x = 1;
    let mut y = 2;
    let mut z;
    let mut sum= 0;

    while y < bound {
        if y % 2 == 0 {
            sum += y;
        }

        z = x + y;
        x = y;
        y = z;
    }

    sum
}
