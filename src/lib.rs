//! Hailstone sequence generator library
//! 
//! This library provides functions to compute hailstone sequences,
//! also known as the Collatz conjecture.

/// Computes the next number in the hailstone sequence
/// 
/// The hailstone sequence follows these rules:
/// - If n is even: n/2
/// - If n is odd: 3n+1
/// 
/// # Examples
/// 
/// ```
/// use hailstone::hail;
/// 
/// assert_eq!(hail(1), 4);  // 1 is odd: 3*1+1 = 4
/// assert_eq!(hail(4), 2);  // 4 is even: 4/2 = 2
/// assert_eq!(hail(2), 1);  // 2 is even: 2/2 = 1
/// ```
pub fn hail(n: i16) -> i16 {
    if n & 1 == 1 {
        n * 3 + 1
    } else {
        n / 2
    }
}

/// Computes the previous number in the hailstone sequence (reverse operation)
/// 
/// This attempts to reverse the hailstone operation:
/// - If n is odd: n*2 (assuming it came from an even number)
/// - If n is even: (n-1)/3 (assuming it came from an odd number)
/// 
/// Note: This reverse operation is not always mathematically valid
/// for all sequences, but matches the original C implementation.
/// 
/// # Examples
/// 
/// ```
/// use hailstone::unhail;
/// 
/// assert_eq!(unhail(4), 1);  // 4 is even: (4-1)/3 = 1
/// assert_eq!(unhail(2), 0);  // 2 is even: (2-1)/3 = 0
/// assert_eq!(unhail(1), 2);  // 1 is odd: 1*2 = 2
/// ```
pub fn unhail(n: i16) -> i16 {
    if n & 1 == 1 {
        n * 2
    } else {
        (n - 1) / 3
    }
}

/// Generates a hailstone sequence starting from the given number
/// 
/// # Examples
/// 
/// ```
/// use hailstone::generate_sequence;
/// 
/// let sequence = generate_sequence(3, 5);
/// assert_eq!(sequence, vec![10, 5, 16, 8, 4]);
/// ```
pub fn generate_sequence(start: i16, count: usize) -> Vec<i16> {
    let mut sequence = Vec::with_capacity(count);
    let mut current = start;
    
    for _ in 0..count {
        current = hail(current);
        sequence.push(current);
    }
    
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hail_even() {
        assert_eq!(hail(4), 2);
        assert_eq!(hail(8), 4);
        assert_eq!(hail(16), 8);
    }

    #[test]
    fn test_hail_odd() {
        assert_eq!(hail(1), 4);
        assert_eq!(hail(3), 10);
        assert_eq!(hail(5), 16);
    }

    #[test]
    fn test_unhail() {
        assert_eq!(unhail(4), 1);  // 4 is even: (4-1)/3 = 1
        assert_eq!(unhail(2), 0);  // 2 is even: (2-1)/3 = 0
        assert_eq!(unhail(1), 2);  // 1 is odd: 1*2 = 2
    }

    #[test]
    fn test_generate_sequence() {
        let sequence = generate_sequence(1, 3);
        assert_eq!(sequence, vec![4, 2, 1]);
    }

    #[test]
    fn test_classic_sequence() {
        // Test the classic 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1 sequence
        let sequence = generate_sequence(3, 7);
        assert_eq!(sequence, vec![10, 5, 16, 8, 4, 2, 1]);
    }
}
