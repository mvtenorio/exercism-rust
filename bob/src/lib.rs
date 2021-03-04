enum SentenceKind {
    Silence,
    Yell,
    ImpoliteQuestion,
    PoliteQuestion,
    Other,
}

#[derive(Copy, Clone)]
struct Sentence {
    message: &'static str,
}

impl Sentence {
    fn new(message: &'static str) -> Self {
        Sentence {
            message: message.trim(),
        }
    }

    fn kind(&self) -> SentenceKind {
        if self.message.is_empty() {
            return SentenceKind::Silence;
        }

        if self.is_inpolite_question() {
            return SentenceKind::ImpoliteQuestion;
        }

        if self.is_polite_question() {
            return SentenceKind::PoliteQuestion;
        }

        if self.is_yell() {
            return SentenceKind::Yell;
        }

        SentenceKind::Other
    }

    fn is_inpolite_question(self) -> bool {
        self.message.chars().any(|c| c.is_alphabetic())
            && self
                .message
                .chars()
                .all(|c| !c.is_alphabetic() || c.is_uppercase())
            && self.message.chars().last().unwrap() == '?'
    }

    fn is_polite_question(self) -> bool {
        self.message.chars().last().unwrap() == '?'
    }

    fn is_yell(self) -> bool {
        self.message.chars().any(|c| c.is_alphabetic())
            && self
                .message
                .chars()
                .all(|c| !c.is_alphabetic() || c.is_uppercase())
    }
}

pub fn reply(message: &'static str) -> &str {
    let sentence = Sentence::new(message);

    match sentence.kind() {
        SentenceKind::Silence => "Fine. Be that way!",
        SentenceKind::ImpoliteQuestion => "Calm down, I know what I'm doing!",
        SentenceKind::PoliteQuestion => "Sure.",
        SentenceKind::Yell => "Whoa, chill out!",
        SentenceKind::Other => "Whatever.",
    }
}
