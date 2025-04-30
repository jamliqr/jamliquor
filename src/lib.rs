//! JamLiquor: A Cleanroom JAM Client with Edge, AI, and PQC Extensions
//!
//! This platform focuses on lightweight design, decentralization, and post-quantum cryptography.

pub mod importer;
pub mod schema;
pub mod state;

pub use importer::Importer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
