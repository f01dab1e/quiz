use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File {
    pub theme: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub program: String,
    pub answer: String,
    pub distractors: Vec<String>,
}
