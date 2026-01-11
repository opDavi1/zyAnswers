use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Gets activity answers from zyBooks", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "TOKEN")]
    pub auth_token: String,

    #[arg(short, long)]
    pub chapter: u32,

    #[arg(short, long)]
    pub section: u32,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Get the most verbose output, enables --verbose automatically
    #[arg(long, default_value_t = false)]
    pub very_verbose: bool,

    #[arg(short, long, value_name = "CODE")]
    pub zybook_code: String,
}
