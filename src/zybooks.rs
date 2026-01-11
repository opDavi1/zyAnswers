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

enum ActivityType<'a> {
    MultipleChoice(&'a Value),
    ShortAnswer(&'a Value),
}

pub struct QuestionParser {
    data: Value,
}

impl QuestionParser {
    pub fn new(data: Value) -> Self {
        Self { data }
    }

    pub fn print_answers(&self) {
        let Some(activities) = self.get_multiple_choice_activities() else {
            return;
        };

        for activity in activities {
            self.print_activity_answers(activity);
        }
    }

    fn get_multiple_choice_activities(&self) -> Option<Vec<&Value>> {
        let content_resources = self.data.get("section")?.get("content_resources")?;

        let arr = content_resources.as_array()?;

        Some(
            arr.iter()
                .filter(|obj| obj["type"] == "multiple_choice")
                .collect(),
        )
    }

    fn print_activity_answers(&self, activity: &Value) {
        if let Some(caption) = activity.get("caption") {
            println!("{}", caption);
        }

        let questions = self.get_questions(activity);

        for question in questions {
            if let Some(answer) = self.find_correct_answer(question) {
                println!("Answer: {}", answer);
            }
        }
    }

    fn get_questions<'a>(&self, activity: &'a Value) -> Vec<&'a Value> {
        activity
            .get("payload")
            .and_then(|p| p.get("questions"))
            .and_then(|q| q.as_array())
            .map(|arr| arr.iter().collect())
            .unwrap_or_default()
    }

    fn find_correct_answer<'a>(&self, question: &'a Value) -> Option<&'a Value> {
        let choices = question.get("choices")?.as_array()?;

        choices
            .iter()
            .find(|choice| choice["correct"] == true)
            .and_then(|choice| choice.get("label"))
    }
}
