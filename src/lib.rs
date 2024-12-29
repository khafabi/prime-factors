pub fn factors(n: u64) -> Vec<u64> {
    fn prime_factors_recursive(n: u64, divisor: u64, accumulator: Vec<u64>) -> Vec<u64> {
        match n {
            1 => accumulator,
            _ if { n % divisor == 0 } => {
                prime_factors_recursive(n / divisor, divisor, [accumulator, vec![divisor]].concat())
            }

            _ => prime_factors_recursive(n, divisor + 1, accumulator),
        }
    }

    if n < 2 {
        vec![]
    } else {
        prime_factors_recursive(n, 2, vec![])
    }
}
