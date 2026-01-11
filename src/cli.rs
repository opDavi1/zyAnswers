use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub auth_token: String,

    #[arg(short, long)]
    pub chapter: u32,

    #[arg(short, long)]
    pub section: u32,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(short, long)]
    pub zybook_code: String,
}
