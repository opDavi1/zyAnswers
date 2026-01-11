use clap::Parser;
use serde_json::Value;
mod cli;
mod zybooks;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();

    let mut data = zybooks::get_zybooks_data(&args)?;

    let content_resources = data["section"]["content_resources"].take();
    if let Value::Array(arr) = content_resources {
        let activity_objects: Vec<&Value> = arr
            .iter()
            .filter(|obj| obj["type"] == "multiple_choice")
            .collect();

        for obj in activity_objects {
            println!("{}", obj["caption"]);

            if let Value::Array(arr) = obj
                .get("payload")
                .unwrap_or_default()
                .get("questions")
                .unwrap_or_default()
            {
                for q in arr {
                    let choices = q.get("choices").unwrap_or_default();
                    let answer: &Value;
                    if let Value::Array(arr) = choices {
                        answer = arr
                            .iter()
                            .filter(|choice| choice["correct"] == true)
                            .nth(0)
                            .unwrap_or_default()
                            .get("label")
                            .unwrap_or_default();
                        println!("Answer: {}", answer);
                    }
                }
            }
        }
    }

    Ok(())
}
