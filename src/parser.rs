use crate::types::{ArgValue, SuiteConfig};
use std::error::Error;
use std::fs;

pub fn from_filepath(filepath: &str) -> Result<SuiteConfig, Box<dyn Error>> {
    let check_config: String = fs::read_to_string(&filepath)?;
    Ok(serde_yaml::from_str(&check_config)?)
}
