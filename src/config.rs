/*
    Talking clock
    Command line application which says the time.

    Copyright 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    voice: String,
    locale: String,
    digital: bool,
    _24hr: bool,
}

impl Config {
    pub fn new(voice: &str, locale: &str, digital: bool, _24hr: bool) -> Config {
        Config {
            voice: voice.to_owned(),
            locale: locale.to_owned(),
            digital: digital,
            _24hr: _24hr,
        } 
    }
    pub fn get_voice(&self) -> &str {
        &self.voice
    }
    pub fn get_locale(&self) -> &str {
        &self.locale
    }
    pub fn is_digital(&self) -> bool {
        self.digital
    }
    pub fn is_24hr(&self) -> bool {
        self._24hr
    }
}
