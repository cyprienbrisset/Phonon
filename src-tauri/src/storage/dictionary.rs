use crate::types::DictionaryData;
use std::fs;
use std::path::PathBuf;

fn dictionary_path() -> PathBuf {
    super::get_app_data_dir().join("dictionary.json")
}

pub fn load_dictionary() -> DictionaryData {
    let path = dictionary_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        DictionaryData::default()
    }
}

pub fn save_dictionary(data: &DictionaryData) -> Result<(), String> {
    super::ensure_app_data_dir().map_err(|e| e.to_string())?;
    let path = dictionary_path();
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

pub fn add_word(word: String) -> Result<(), String> {
    let mut data = load_dictionary();
    if !data.words.contains(&word) {
        data.words.push(word);
        save_dictionary(&data)?;
    }
    Ok(())
}

pub fn remove_word(word: &str) -> Result<(), String> {
    let mut data = load_dictionary();
    data.words.retain(|w| w != word);
    save_dictionary(&data)
}
