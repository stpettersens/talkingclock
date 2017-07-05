#[derive(Deserialize)]
pub struct DefaultLocale {
    locale: String,
}

impl DefaultLocale {
    pub fn get_locale(&self) -> String {
        self.locale.clone()
    }
}

#[derive(Deserialize)]
pub struct LocaleMessage {
    locstr: String,
    message: String,
    phonetic: String,
}

impl LocaleMessage {
    fn get_str(&self) -> String {
        self.locstr.clone()
    }
    fn get_message(&self) -> String {
        self.message.clone()
    }
    fn get_phonetic(&self) -> String {
        self.phonetic.clone()
    }
}

pub struct Locale {
    messages: Vec<LocaleMessage>,
}

impl Locale {
    pub fn new() -> Locale {
        Locale {
            messages: Vec::new(),
        }
    }
    pub fn add_message(&mut self, message: LocaleMessage) {
        self.messages.push(message);
    }
    fn get_message_str(&self, locstr: &str) -> String {
        let mut message_str = String::new();
        for message in &self.messages {
            if message.get_str() == locstr {
                message_str = message.get_message();
            }
        }
        message_str
    }
    fn get_phonetic_str(&self, locstr: &str) -> String {
        let mut phonetic_str = String::new();
        for message in &self.messages {
            if message.get_str() == locstr {
                phonetic_str = message.get_phonetic();
            }
        }
        phonetic_str
    }
}

pub fn localize(message: &str, locale: &Locale) -> String {
    let mut localized: Vec<String> = Vec::new();
    let split = message.split(" ");
    let locstrs: Vec<&str> = split.collect();
    if !locale.get_message_str(locstrs[0]).is_empty() {
        for l in locstrs {
            localized.push(locale.get_message_str(l));
        }
    } else {
        return message.to_owned();
    }
    localized.join(" ")
}

pub fn phoneticize(message: &str, locale: &Locale) -> String {
    let mut phoneticized: Vec<String> = Vec::new();
    let split = message.split(" ");
    let locstrs: Vec<&str> = split.collect();
    if !locale.get_message_str(locstrs[0]).is_empty() {
        for l in locstrs {
            phoneticized.push(locale.get_phonetic_str(l));
        }
    } else {
        return message.to_owned();
    }
    phoneticized.join(" ")
}
