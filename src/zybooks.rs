use crate::cli;
use reqwest::blocking::Client;
use serde_json::Value;

const BASE_URL: &str = "https://zyserver.zybooks.com/v1/zybook";

pub fn get_zybooks_data(args: &cli::Cli) -> Result<Value, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}/{}/chapter/{}/section/{}",
        BASE_URL, args.zybook_code, args.chapter, args.section
    );

    let client = Client::new();
    println!("Sending request to server: {}", url);
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .header("Accept-Encoding", "gzip")
        .header("Authorization", format!("Bearer {}", args.auth_token))
        .header("Host", "zyserver.zybooks.com")
        /* // Imitate firefox if necessary (i.e. if your requests get blocked)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
        )*/
        .send()?;
    println!("Status: {}", response.status());
    let data = response.json()?;
    Ok(data)
}
