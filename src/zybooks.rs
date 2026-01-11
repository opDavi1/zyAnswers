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
    if args.verbose {
        println!("Sending request to server: {}", url);
    }
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
    if args.verbose {
        println!(
            "Status: {}, {:#?}",
            response.status(),
            response.status().canonical_reason()
        );
        println!("Response Headers: {:#?}", response.headers());
    }
    let data = response.json()?;
    Ok(data)
}

enum ActivityType {
    MultipleChoice,
    ShortAnswer,
    Unknown,
}

impl ActivityType {
    fn as_string(&self) -> &str {
        match self {
            ActivityType::MultipleChoice => "multiple_choice",
            ActivityType::ShortAnswer => "short_answer",
            _ => "unknown",
        }
    }
}

impl From<&str> for ActivityType {
    fn from(value: &str) -> Self {
        match value {
            "multiple_choice" => ActivityType::MultipleChoice,
            "short_answer" => ActivityType::ShortAnswer,
            _ => ActivityType::Unknown,
        }
    }
}

pub struct QuestionParser {
    data: Value,
}

impl QuestionParser {
    pub fn new(data: Value) -> Self {
        Self { data }
    }

    pub fn print_answers(&self) {
        let multiple_choice_activities = self.get_activities_by_type(ActivityType::MultipleChoice);

        let short_answer_activities = self.get_activities_by_type(ActivityType::ShortAnswer);

        if let Some(activities) = multiple_choice_activities {
            println!("\nMultiple Choice:\n----------");
            for activity in activities {
                self.print_activity_answers(activity, ActivityType::MultipleChoice);
                println!("");
            }
        }

        if let Some(activities) = short_answer_activities {
            println!("\nShort Answers:\n----------");
            for activity in activities {
                self.print_activity_answers(activity, ActivityType::ShortAnswer);
                println!("");
            }
        }
    }

    fn get_activities_by_type(&self, activity_type: ActivityType) -> Option<Vec<&Value>> {
        let content_resources = self.data.get("section")?.get("content_resources")?;

        let arr = content_resources.as_array()?;

        let activities: Vec<&Value> = arr
            .iter()
            .filter(|obj| obj["type"] == activity_type.as_string())
            .collect();

        if activities.len() > 0 {
            return Some(activities);
        } else {
            return None;
        }
    }

    fn print_activity_answers(&self, activity: &Value, activity_type: ActivityType) {
        if let Some(caption) = activity.get("caption") {
            println!("{}", caption);
        }

        let questions = self.get_questions(activity);

        for question in questions {
            if let Some(answer) = self.find_correct_answer(question, &activity_type) {
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

    fn find_correct_answer<'a>(
        &self,
        question: &'a Value,
        activity_type: &ActivityType,
    ) -> Option<&'a Value> {
        match activity_type {
            ActivityType::MultipleChoice => {
                let choices = question.get("choices")?.as_array()?;
                return choices
                    .iter()
                    .find(|choice| choice["correct"] == true)
                    .and_then(|choice| choice.get("label"));
            }
            ActivityType::ShortAnswer => {
                return question.get("answers")?.as_array()?.first();
            }
            _ => return None,
        }
    }
}
