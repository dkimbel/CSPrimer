/// Recursively calculate an exponent in a sublinear number of steps, by splitting
/// the power roughly evenly into two recursive calls. For instance, our first step
/// in calculating 2^7 would be converting it to 2^3 * 2^4.
pub fn raise_to_power(base: u64, power: u32) -> u64 {
    if power == 0 {
        return 1;
    } else if power == 1 {
        return base;
    }

    let left_power = power >> 1; // fancy division by 2
    let right_power = power - left_power;
    return raise_to_power(base, left_power) * raise_to_power(base, right_power);
}

/// Calculate an exponent in linear time, just by repeatedly multiplying
/// the base by itself.
pub fn raise_to_power_linear_iterative(base: u64, mut power: u32) -> u64 {
    let mut result = 1;

    while power > 0 {
        result *= base;
        power -= 1;
    }

    result
}

/// Calculate an exponent in linear time by repeatedly mulitpying the base
/// by itself, recursively.
pub fn raise_to_power_linear_recursive(base: u64, power: u32) -> u64 {
    if power == 0 {
        return 1;
    }

    base * raise_to_power_linear_recursive(base, power - 1)
}

/// I didn't come up with this one myself; it's from Oz's instructional video. It's also
/// very similar to the actual implementation of `pow` in Rust's stdlib:
/// https://github.com/rust-lang/rust/blob/4e59b1da80db461d0c48b43142c9f82df122922a/library/core/src/num/uint_macros.rs#L2928-L2981
pub fn raise_to_power_fast_iterative(mut base: u64, mut power: u32) -> u64 {
    let mut result = 1;

    while power > 0 {
        if power & 1 == 1 {
            result *= base;
            power -= 1;
        } else {
            // 'Power is even' case, where we halve the power and square the base. One
            // example is converting 2^6 to 4^3 -- or more dramatically, 2^60 to 4^30.
            base *= base;
            power >>= 1;
        }
    }

    result
}

/// Just like iterative -- not my idea, rather from Oz's video.
pub fn raise_to_power_fast_recursive(base: u64, power: u32) -> u64 {
    match power {
        0 => 1,
        1 => base,
        n if n & 1 == 1 => base * raise_to_power_fast_recursive(base, power - 1),
        // 'power is even' case, where we halve the power and square the base
        _ => raise_to_power_fast_recursive(base * base, power >> 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(raise_to_power(2, 0), 1);
        assert_eq!(raise_to_power_linear_recursive(2, 0), 1);
        assert_eq!(raise_to_power_linear_iterative(2, 0), 1);
        assert_eq!(raise_to_power_fast_iterative(2, 0), 1);
        assert_eq!(raise_to_power_fast_recursive(2, 0), 1);
    }

    #[test]
    fn one() {
        assert_eq!(raise_to_power(2, 1), 2);
        assert_eq!(raise_to_power_linear_recursive(2, 1), 2);
        assert_eq!(raise_to_power_linear_iterative(2, 1), 2);
        assert_eq!(raise_to_power_fast_iterative(2, 1), 2);
        assert_eq!(raise_to_power_fast_recursive(2, 1), 2);
    }

    #[test]
    fn large_even() {
        assert_eq!(raise_to_power(2, 20), 2u64.pow(20));
        assert_eq!(raise_to_power_linear_recursive(2, 20), 2u64.pow(20));
        assert_eq!(raise_to_power_linear_iterative(2, 20), 2u64.pow(20));
        assert_eq!(raise_to_power_fast_iterative(2, 20), 2u64.pow(20));
        assert_eq!(raise_to_power_fast_recursive(2, 20), 2u64.pow(20));
    }

    #[test]
    fn large_odd() {
        assert_eq!(raise_to_power(2, 21), 2u64.pow(21));
        assert_eq!(raise_to_power_linear_recursive(2, 21), 2u64.pow(21));
        assert_eq!(raise_to_power_linear_iterative(2, 21), 2u64.pow(21));
        assert_eq!(raise_to_power_fast_iterative(2, 21), 2u64.pow(21));
        assert_eq!(raise_to_power_fast_recursive(2, 21), 2u64.pow(21));
    }

    #[test]
    fn base_10() {
        assert_eq!(raise_to_power(10, 5), 100000);
        assert_eq!(raise_to_power_linear_recursive(10, 5), 100000);
        assert_eq!(raise_to_power_linear_iterative(10, 5), 100000);
        assert_eq!(raise_to_power_fast_iterative(10, 5), 100000);
        assert_eq!(raise_to_power_fast_recursive(10, 5), 100000);
    }
}
