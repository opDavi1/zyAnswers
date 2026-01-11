use clap::Parser;

mod cli;
mod zybooks;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = cli::Cli::parse();
    if args.very_verbose {
        args.verbose = true;
    }
    let data = zybooks::get_zybooks_data(&args)?;

    let parser = zybooks::QuestionParser::new(data);
    parser.print_answers();

    Ok(())
}
