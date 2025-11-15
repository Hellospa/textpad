use std::fs;
use std::collections::HashMap;

pub struct MutiLang {
    current_lang: String,
    data: HashMap<String, HashMap<String, String>>,
}

impl MutiLang {
    pub fn load(lang: &str, langs_dir: &str) -> Self {
        let entries = fs::read_dir(langs_dir).expect(&format!("Failed to read lang dir: {}", langs_dir));
        let mut data = HashMap::new();

        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_file() {
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                let content = fs::read(&path).expect(&format!("Failed to read lang file: {:?}", &path));
                data.insert(file_name.to_string(), toml::from_slice(&content).unwrap());
            }
        }
        Self {
            current_lang: lang.to_string(),
            data
        }
    }
    pub fn get_text(&self, key: &str) -> Option<&str> {
        let lang_map = self.data.get(&self.current_lang)?;
        lang_map.get(key).map(|s| s.as_str())
    }
    pub fn set_lang(&mut self, lang: &str) -> bool {
        if self.data.contains_key(lang) {
            self.current_lang = lang.to_string();
            true
        } else {
            false
        }
    }
}