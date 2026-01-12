use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(version, about = "Gets activity answers from zyBooks", long_about = None)]
pub struct Cli {
    /// select the chapter you want
    #[arg(short, long)]
    pub chapter: u32,

    #[command(flatten)]
    pub login: Login,

    /// used with --username
    #[arg(short, long, requires = "username")]
    pub password: Option<String>,

    /// select the section you want answers for
    #[arg(short, long)]
    pub section: u32,

    /// get more verbose output, good for debugging
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Get the most verbose output, enables --verbose automatically, MAY PRINT SENSITIVE DATA TO
    /// TERMINAL
    #[arg(long, default_value_t = false)]
    pub very_verbose: bool,

    /// The code for your zyBook (Ex: WebProgrammingR37)
    #[arg(short, long, value_name = "CODE")]
    pub zybook_code: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Login {
    /// requires --password, zyAnswers will obtain an auth token with your credentials
    #[arg(short, long, requires = "password")]
    pub username: Option<String>,

    /// provide an auth token directly to avoid sending a login request
    #[arg(short, long, value_name = "TOKEN")]
    pub auth_token: Option<String>,
}
