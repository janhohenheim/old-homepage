pub struct Question {
    pub question: String,
    pub wrong_answers: Vec<Answer>,
    pub correct_answer: Answer,
}

pub struct Answer {
    text: String,
    times_chosen: u32,
}

pub struct Player {}

pub struct Game {}
