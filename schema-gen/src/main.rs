use std::fs::File;
use std::io::Write;
use anyhow::Result;
use clap::Parser;
use schemars::schema_for;
use oci_compat_schema::ZoneCompatMetadata;

#[derive(Parser)]
struct Args {
    /// Location where to generate the schema file
    schema_file: String,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let schema = schema_for!(ZoneCompatMetadata);
    let out_string = serde_json::to_string_pretty(&schema)?;
    let mut f = File::create(&args.schema_file)?;
    f.write_all(out_string.as_bytes())?;
    Ok(())
}

