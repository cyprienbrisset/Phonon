use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    OpenVINOInitFailed(String),
    ModelLoadFailed(String),
    InferenceError(String),
    AudioTooShort,
    InvalidSampleRate(u32),
    VocabularyError(String),
    TensorError(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::OpenVINOInitFailed(msg) => write!(f, "OpenVINO initialization failed: {}", msg),
            EngineError::ModelLoadFailed(msg) => write!(f, "Model loading failed: {}", msg),
            EngineError::InferenceError(msg) => write!(f, "Inference error: {}", msg),
            EngineError::AudioTooShort => write!(f, "Audio too short (minimum 0.5 seconds)"),
            EngineError::InvalidSampleRate(rate) => write!(f, "Invalid sample rate: {}Hz (expected 16000Hz)", rate),
            EngineError::VocabularyError(msg) => write!(f, "Vocabulary error: {}", msg),
            EngineError::TensorError(msg) => write!(f, "Tensor error: {}", msg),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<EngineError> for String {
    fn from(err: EngineError) -> String {
        err.to_string()
    }
}
