//! Minimal package for the trusted publishing demo.

/// Adds two unsigned integers if the result fits into `u64`.
pub fn checked_add(a: u64, b: u64) -> Option<u64> {
    a.checked_add(b)
}

#[cfg(test)]
mod tests {
    use super::checked_add;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(checked_add(2, 3), Some(5));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(checked_add(u64::MAX, 1), None);
    }
}
