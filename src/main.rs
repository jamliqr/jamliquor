mod importer;
mod schema;

use anyhow::{Context, Result};
use clap::Parser;
use importer::Importer;
use log::info;
use std::path::PathBuf;

/// JamLiquor CLI for blockchain block importing
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the block JSON file
    #[arg(short, long, default_value = "tests/vectors/codec/data/block.json")]
    block_path: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn setup_logging(verbose: bool) {
    let log_level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::Builder::new().filter_level(log_level).init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    let mut importer = Importer::new();
    let block = importer
        .import_block(&cli.block_path)
        .with_context(|| format!("Failed to import block from {}", cli.block_path.display()))?;

    info!("Successfully imported block: {:?}", block);
    info!("Importer state: {:?}", importer.state());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_defaults() {
        let default_path = PathBuf::from("tests/vectors/codec/data/block.json");
        assert!(default_path.exists(), "Default block vector should exist");
    }

    #[test]
    fn test_importer_initialization() {
        let _importer = Importer::new();
        // TODO: Add appropriate state initialization check
    }

    #[test]
    fn test_block_import() -> Result<()> {
        let mut importer = Importer::new();

        // Set initial state to match block.json's parent
        importer.set_initial_state(
            [
                0x5c, 0x74, 0x3d, 0xbc, 0x51, 0x42, 0x84, 0xb2, 0xea, 0x57, 0x79, 0x87, 0x87, 0xc5,
                0xa1, 0x55, 0xef, 0x9d, 0x7a, 0xc1, 0xe9, 0x49, 0x9e, 0xc6, 0x59, 0x10, 0xa7, 0xa3,
                0xd6, 0x58, 0x97, 0xb7,
            ], // parent
            [
                0x25, 0x91, 0xeb, 0xd0, 0x47, 0x48, 0x9f, 0x10, 0x06, 0x36, 0x1a, 0x42, 0x54, 0x73,
                0x14, 0x66, 0xa9, 0x46, 0x17, 0x4a, 0xf0, 0x2f, 0xe1, 0xd8, 0x66, 0x81, 0xd2, 0x54,
                0xcf, 0xd4, 0xa0, 0x0b,
            ], // parent_state_root
        );

        let block_path = PathBuf::from("tests/vectors/codec/data/block.json");
        let block = importer.import_block(&block_path)?;

        // Verify the block was imported successfully
        assert!(block.header.slot > 0, "Block slot should be positive");
        assert_eq!(importer.state().get_last_slot(), block.header.slot as u64);

        // Get ticket stats
        let (total_tickets, valid_tickets, invalid_tickets) = importer.state().get_ticket_stats();

        // Verify ticket counts
        if let Some(tickets_mark) = &block.header.tickets_mark {
            assert_eq!(
                total_tickets,
                tickets_mark.len() as u64,
                "Total tickets count mismatch"
            );
            assert_eq!(
                valid_tickets + invalid_tickets,
                total_tickets,
                "Valid + invalid tickets should equal total"
            );
        }

        // Count valid preimages (non-empty blob)
        let valid_preimage_count = block
            .extrinsic
            .preimages
            .iter()
            .filter(|preimage| !preimage.blob.is_empty())
            .count() as u64;

        // Verify total counter equals valid tickets plus valid preimages
        assert_eq!(
            importer.state().get_counter(),
            valid_tickets + valid_preimage_count,
            "Counter should equal valid tickets + valid preimages"
        );

        // Explicit author_index check
        if let Some(epoch_mark) = &block.header.epoch_mark {
            assert!(
                block.header.author_index as usize <= epoch_mark.validators.len() - 1,
                "Author index {} exceeds validator count {}",
                block.header.author_index,
                epoch_mark.validators.len()
            );
        }

        // Explicit tickets_mark check
        if let Some(tickets_mark) = &block.header.tickets_mark {
            assert_eq!(
                tickets_mark.len(),
                block.extrinsic.tickets.len(),
                "Tickets mark count ({}) mismatch with extrinsic tickets count ({})",
                tickets_mark.len(),
                block.extrinsic.tickets.len()
            )
        }

        // Explicit offenders_mark check
        assert!(
            !block.header.offenders_mark.is_empty(),
            "Offenders mark cannot be empty"
        );

        // Explicit seal check
        assert!(!block.header.seal.is_empty(), "Seal cannot be empty");

        // Explicit entropy_source check
        assert!(
            !block.header.entropy_source.is_empty(),
            "Entropy source cannot be empty"
        );

        Ok(())
    }
}
