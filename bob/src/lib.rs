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

    fn reply(&self) -> &'static str {
        match self.message {
            m if m.is_empty() => "Fine. Be that way!",
            _ if self.is_impolite_question() => "Calm down, I know what I'm doing!",
            _ if self.is_polite_question() => "Sure.",
            _ if self.is_yell() => "Whoa, chill out!",
            _ => "Whatever.",
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
    sentence.reply()
}
