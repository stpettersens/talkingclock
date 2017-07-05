pub struct Config {
    voice: String,
    locale: String,
}

impl Config {
    pub fn new(voice: &str, locale: &str) -> Config {
        Config {
            voice: voice.to_owned(),
            locale: locale.to_owned(),
        } 
    }
    pub fn get_voice(&self) -> &str {
        &self.voice
    }
    pub fn get_locale(&self) -> &str {
        &self.locale
    }
}
