use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ZoneCompatMetadata {
    pub libc_version: Version,
    pub fork: String,
    pub gate_src_hash: Option<String>,
    pub distribution: String,
}