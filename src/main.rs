mod evaluator;
mod parser;
mod types;

use evaluator::execute_suite;
use parser::from_filepath;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let default_config_path = "test.yaml";
    let binding = default_config_path.to_string();
    let filepath = args.get(1).unwrap_or(&binding);

    let parsed_config = from_filepath(filepath)?;
    println!("===\n PARSED CONFIG\n{:?}\n===", parsed_config);

    println!("===\n EVALUATED SUITE \n===");
    execute_suite(&parsed_config);
    Ok(())
}
