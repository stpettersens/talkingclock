use std::path::PathBuf;

fn get_delim(path: &str) -> &str {
    let mut delim = "/";
    if path.contains(r"\") {
        delim = r"\";
    }
    &delim
}

fn split_path(path: &str) -> String {
    let delim = get_delim(path);
    let split = path.split(delim);
    let mut spath: Vec<&str> = split.collect();
    spath.pop();
    spath.join(delim).to_owned()
}

#[derive(Debug)]
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

    pub fn set_paths(&mut self, path: PathBuf) {
        let spath = format!("{}", path.display());
        self.voice = format!("{}{}{}",
        split_path(&spath), get_delim(&spath), self.voice);
        self.locale = format!("{}{}{}",
        split_path(&spath), get_delim(&spath), self.locale);
    }
    
    pub fn get_voice(&self) -> &str {
        &self.voice
    }
}
