import { useEffect, useState } from 'react';
import { Settings, X, Plus, Trash2 } from 'lucide-react';
import { useSettingsStore } from '../stores/settingsStore';

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SettingsPanel({ isOpen, onClose }: SettingsPanelProps) {
  const { settings, devices, dictionary, loadSettings, loadDevices, loadDictionary, updateSettings, addWord, removeWord } = useSettingsStore();
  const [newWord, setNewWord] = useState('');

  useEffect(() => {
    if (isOpen) {
      loadSettings();
      loadDevices();
      loadDictionary();
    }
  }, [isOpen, loadSettings, loadDevices, loadDictionary]);

  const handleAddWord = async () => {
    if (newWord.trim()) {
      await addWord(newWord.trim());
      setNewWord('');
    }
  };

  if (!isOpen || !settings) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex justify-end z-50">
      <div className="bg-white dark:bg-gray-900 w-full max-w-md h-full overflow-y-auto">
        <div className="sticky top-0 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 p-4 flex justify-between items-center">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Settings className="w-5 h-5" />
            Paramètres
          </h2>
          <button onClick={onClose} className="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg">
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-4 space-y-6">
          {/* Microphone */}
          <div>
            <label className="block text-sm font-medium mb-2">Microphone</label>
            <select
              value={settings.microphone_id || ''}
              onChange={(e) => updateSettings({ microphone_id: e.target.value || null })}
              className="w-full p-2 border rounded-lg dark:bg-gray-800 dark:border-gray-700"
            >
              <option value="">Par défaut</option>
              {devices.map((device) => (
                <option key={device.id} value={device.id}>
                  {device.name} {device.is_default ? '(défaut)' : ''}
                </option>
              ))}
            </select>
          </div>

          {/* Langue */}
          <div>
            <label className="block text-sm font-medium mb-2">Langue de transcription</label>
            <select
              value={settings.transcription_language}
              onChange={(e) => updateSettings({ transcription_language: e.target.value })}
              className="w-full p-2 border rounded-lg dark:bg-gray-800 dark:border-gray-700"
            >
              <option value="fr">Français</option>
              <option value="en">English</option>
              <option value="de">Deutsch</option>
              <option value="es">Español</option>
              <option value="auto">Auto-détection</option>
            </select>
          </div>

          {/* Thème */}
          <div>
            <label className="block text-sm font-medium mb-2">Thème</label>
            <select
              value={settings.theme}
              onChange={(e) => updateSettings({ theme: e.target.value as 'light' | 'dark' | 'system' })}
              className="w-full p-2 border rounded-lg dark:bg-gray-800 dark:border-gray-700"
            >
              <option value="system">Système</option>
              <option value="light">Clair</option>
              <option value="dark">Sombre</option>
            </select>
          </div>

          {/* Options */}
          <div className="space-y-3">
            <label className="flex items-center gap-3">
              <input
                type="checkbox"
                checked={settings.auto_copy_to_clipboard}
                onChange={(e) => updateSettings({ auto_copy_to_clipboard: e.target.checked })}
                className="w-4 h-4"
              />
              <span>Copier automatiquement dans le presse-papier</span>
            </label>

            <label className="flex items-center gap-3">
              <input
                type="checkbox"
                checked={settings.notification_on_complete}
                onChange={(e) => updateSettings({ notification_on_complete: e.target.checked })}
                className="w-4 h-4"
              />
              <span>Notification à la fin de la transcription</span>
            </label>

            <label className="flex items-center gap-3">
              <input
                type="checkbox"
                checked={settings.minimize_to_tray}
                onChange={(e) => updateSettings({ minimize_to_tray: e.target.checked })}
                className="w-4 h-4"
              />
              <span>Minimiser dans la barre système</span>
            </label>
          </div>

          {/* Raccourcis */}
          <div>
            <label className="block text-sm font-medium mb-2">Raccourcis clavier</label>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between p-2 bg-gray-100 dark:bg-gray-800 rounded">
                <span>Push-to-talk</span>
                <kbd className="px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded text-xs">
                  {settings.hotkey_push_to_talk}
                </kbd>
              </div>
              <div className="flex justify-between p-2 bg-gray-100 dark:bg-gray-800 rounded">
                <span>Toggle record</span>
                <kbd className="px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded text-xs">
                  {settings.hotkey_toggle_record}
                </kbd>
              </div>
            </div>
          </div>

          {/* Dictionnaire */}
          <div>
            <label className="block text-sm font-medium mb-2">Dictionnaire personnalisé</label>
            <div className="flex gap-2 mb-2">
              <input
                type="text"
                value={newWord}
                onChange={(e) => setNewWord(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleAddWord()}
                placeholder="Ajouter un mot..."
                className="flex-1 p-2 border rounded-lg dark:bg-gray-800 dark:border-gray-700"
              />
              <button
                onClick={handleAddWord}
                className="p-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
              >
                <Plus className="w-5 h-5" />
              </button>
            </div>
            <div className="flex flex-wrap gap-2">
              {dictionary.map((word) => (
                <span
                  key={word}
                  className="inline-flex items-center gap-1 px-2 py-1 bg-gray-100 dark:bg-gray-800 rounded"
                >
                  {word}
                  <button onClick={() => removeWord(word)} className="hover:text-red-500">
                    <Trash2 className="w-3 h-3" />
                  </button>
                </span>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
