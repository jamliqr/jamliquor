//! JamLiquor: A decentralized blockchain platform
//!
//! This platform focuses on lightweight design, decentralization, and post-quantum cryptography.

pub mod schema;
pub mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
