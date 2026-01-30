import { Mic, MicOff, Loader2 } from 'lucide-react';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useTranscriptionStore } from '../stores/transcriptionStore';
import { useSettingsStore } from '../stores/settingsStore';

export function DictationPanel() {
  const { status, result, error, startRecording, stopRecording, clearError } = useTranscriptionStore();
  const { settings } = useSettingsStore();

  const handleToggle = async () => {
    try {
      if (status === 'recording') {
        const transcription = await stopRecording();
        if (settings?.auto_copy_to_clipboard && transcription.text) {
          await writeText(transcription.text);
        }
      } else if (status === 'idle' || status === 'completed' || status === 'error') {
        await startRecording();
      }
    } catch (err) {
      console.error('Recording error:', err);
    }
  };

  const getButtonClasses = () => {
    const base = 'w-32 h-32 rounded-full flex items-center justify-center shadow-xl transition-all duration-200';
    switch (status) {
      case 'recording':
        return `${base} bg-red-500 hover:bg-red-600 animate-pulse`;
      case 'processing':
        return `${base} bg-gray-400 cursor-not-allowed`;
      default:
        return `${base} bg-blue-500 hover:bg-blue-600 hover:scale-105`;
    }
  };

  const getStatusText = () => {
    switch (status) {
      case 'recording':
        return 'Enregistrement en cours...';
      case 'processing':
        return 'Transcription...';
      case 'completed':
        return 'Transcription terminée';
      case 'error':
        return 'Erreur';
      default:
        return 'Cliquez pour dicter';
    }
  };

  return (
    <div className="flex flex-col items-center justify-center p-8 space-y-8">
      <button
        onClick={handleToggle}
        disabled={status === 'processing'}
        className={getButtonClasses()}
        aria-label={status === 'recording' ? 'Arrêter' : 'Démarrer'}
      >
        {status === 'processing' ? (
          <Loader2 className="w-16 h-16 text-white animate-spin" />
        ) : status === 'recording' ? (
          <MicOff className="w-16 h-16 text-white" />
        ) : (
          <Mic className="w-16 h-16 text-white" />
        )}
      </button>

      <p className="text-lg text-gray-600 dark:text-gray-300">{getStatusText()}</p>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded max-w-md">
          <p>{error}</p>
          <button onClick={clearError} className="text-sm underline mt-2">
            Fermer
          </button>
        </div>
      )}

      {result && status === 'completed' && (
        <div className="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 w-full max-w-lg shadow-lg">
          <p className="text-gray-900 dark:text-gray-100 text-lg leading-relaxed">
            {result.text}
          </p>
          <div className="flex justify-between items-center mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
            <span className="text-sm text-gray-500">
              {result.processing_time_ms}ms • {(result.confidence * 100).toFixed(0)}% confiance
            </span>
            <span className="text-sm text-gray-500">
              {result.duration_seconds.toFixed(1)}s
            </span>
          </div>
        </div>
      )}
    </div>
  );
}
