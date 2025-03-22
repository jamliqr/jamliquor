mod schema;
mod importer;

use anyhow::Result;

fn main() -> Result<()> {
    let importer = importer::Importer::new("../../kd/jamtestvectors");
    let block = importer.import_block("block.json")?;
    println!("Block: {:?}", block);
    Ok(())
}