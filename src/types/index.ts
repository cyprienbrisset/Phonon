export interface TranscriptionResult {
  text: string;
  confidence: number;
  duration_seconds: number;
  processing_time_ms: number;
  detected_language: string | null;
  timestamp: number;
}

export type ModelSize = 'tiny' | 'small' | 'medium';

export interface ModelInfo {
  size: ModelSize;
  display_name: string;
  available: boolean;
  size_bytes: number;
}

export interface DownloadProgress {
  downloaded: number;
  total: number;
  percent: number;
}

export interface AppSettings {
  microphone_id: string | null;
  hotkey_push_to_talk: string;
  hotkey_toggle_record: string;
  transcription_language: string;
  auto_detect_language: boolean;
  theme: 'light' | 'dark' | 'system';
  minimize_to_tray: boolean;
  auto_copy_to_clipboard: boolean;
  notification_on_complete: boolean;
  whisper_model: ModelSize;
}

export interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
}

export type TranscriptionStatus = 'idle' | 'recording' | 'processing' | 'completed' | 'error';
