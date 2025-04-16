//! JamLiquor: A decentralized blockchain platform
//!
//! This platform focuses on lightweight design, decentralization, and post-quantum cryptography.

/// Placeholder function for initial setup
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
