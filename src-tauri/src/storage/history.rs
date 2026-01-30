use crate::types::{HistoryData, TranscriptionResult};
use std::fs;
use std::path::PathBuf;

const MAX_HISTORY: usize = 50;

fn history_path() -> PathBuf {
    super::get_app_data_dir().join("history.json")
}

pub fn load_history() -> HistoryData {
    let path = history_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        HistoryData::default()
    }
}

pub fn save_history(data: &HistoryData) -> Result<(), String> {
    super::ensure_app_data_dir().map_err(|e| e.to_string())?;
    let path = history_path();
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

pub fn add_transcription(result: TranscriptionResult) -> Result<(), String> {
    let mut data = load_history();
    data.transcriptions.insert(0, result);
    data.transcriptions.truncate(MAX_HISTORY);
    save_history(&data)
}

pub fn clear_history() -> Result<(), String> {
    save_history(&HistoryData::default())
}
