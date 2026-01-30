import { useEffect, useState } from 'react';
import { Settings, History } from 'lucide-react';
import { DictationPanel } from './components/DictationPanel';
import { TranscriptionHistory } from './components/TranscriptionHistory';
import { SettingsPanel } from './components/SettingsPanel';
import { useSettingsStore } from './stores/settingsStore';
import { useHotkeys } from './hooks/useHotkeys';

type Tab = 'dictation' | 'history';

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('dictation');
  const [settingsOpen, setSettingsOpen] = useState(false);
  const { loadSettings } = useSettingsStore();

  useHotkeys();

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-4 py-3">
        <div className="flex justify-between items-center">
          <h1 className="text-xl font-bold text-gray-900 dark:text-white">WakaScribe</h1>
          <button
            onClick={() => setSettingsOpen(true)}
            className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
          >
            <Settings className="w-5 h-5 text-gray-600 dark:text-gray-300" />
          </button>
        </div>
      </header>

      {/* Tabs */}
      <nav className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
        <div className="flex">
          <button
            onClick={() => setActiveTab('dictation')}
            className={`flex-1 py-3 text-center font-medium transition-colors ${
              activeTab === 'dictation'
                ? 'text-blue-600 border-b-2 border-blue-600'
                : 'text-gray-500 hover:text-gray-700'
            }`}
          >
            Dictée
          </button>
          <button
            onClick={() => setActiveTab('history')}
            className={`flex-1 py-3 text-center font-medium transition-colors flex items-center justify-center gap-2 ${
              activeTab === 'history'
                ? 'text-blue-600 border-b-2 border-blue-600'
                : 'text-gray-500 hover:text-gray-700'
            }`}
          >
            <History className="w-4 h-4" />
            Historique
          </button>
        </div>
      </nav>

      {/* Content */}
      <main className="container mx-auto max-w-2xl">
        {activeTab === 'dictation' ? <DictationPanel /> : <TranscriptionHistory />}
      </main>

      {/* Settings Panel */}
      <SettingsPanel isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </div>
  );
}

export default App;
