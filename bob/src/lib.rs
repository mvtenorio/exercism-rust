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
        match self.message {
            m if m.is_empty() => SentenceKind::Silence,
            _ if self.is_impolite_question() => SentenceKind::ImpoliteQuestion,
            _ if self.is_polite_question() => SentenceKind::PoliteQuestion,
            _ if self.is_yell() => SentenceKind::Yell,
            _ => SentenceKind::Other,
        }
    }

    fn is_impolite_question(self) -> bool {
        self.message.ends_with("?") && self.is_yell()
    }

    fn is_polite_question(self) -> bool {
        self.message.ends_with("?")
    }

    fn is_yell(self) -> bool {
        self.message.to_uppercase() == self.message
            && self.message.chars().any(|c| c.is_alphabetic())
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
