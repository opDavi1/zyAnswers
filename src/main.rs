use clap::Parser;
use serde_json::Value;

mod cli;
mod zybooks;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();
    let data = zybooks::get_zybooks_data(&args)?;

    let parser = zybooks::QuestionParser::new(data);
    parser.print_answers();

    Ok(())
}
