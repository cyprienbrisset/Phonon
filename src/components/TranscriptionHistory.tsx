import { useEffect } from 'react';
import { Clock, Trash2 } from 'lucide-react';
import { useTranscriptionStore } from '../stores/transcriptionStore';

export function TranscriptionHistory() {
  const { history, loadHistory, clearHistory } = useTranscriptionStore();

  useEffect(() => {
    loadHistory();
  }, [loadHistory]);

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString('fr-FR', {
      day: '2-digit',
      month: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  if (history.length === 0) {
    return (
      <div className="p-8 text-center text-gray-500">
        <Clock className="w-12 h-12 mx-auto mb-4 opacity-50" />
        <p>Aucun historique</p>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
          Historique ({history.length})
        </h2>
        <button
          onClick={clearHistory}
          className="text-red-500 hover:text-red-700 p-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20"
          title="Effacer l'historique"
        >
          <Trash2 className="w-5 h-5" />
        </button>
      </div>

      <div className="space-y-3 max-h-96 overflow-y-auto">
        {history.map((item, index) => (
          <div
            key={`${item.timestamp}-${index}`}
            className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4"
          >
            <p className="text-gray-900 dark:text-gray-100 line-clamp-3">{item.text}</p>
            <div className="flex justify-between mt-2 text-xs text-gray-500">
              <span>{formatDate(item.timestamp)}</span>
              <span>{item.duration_seconds.toFixed(1)}s</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
