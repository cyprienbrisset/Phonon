import { useEffect, useRef } from 'react';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { useTranscriptionStore } from '../stores/transcriptionStore';
import { useSettingsStore } from '../stores/settingsStore';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

export function useHotkeys() {
  const { status, startRecording, stopRecording } = useTranscriptionStore();
  const { settings } = useSettingsStore();
  const statusRef = useRef(status);

  useEffect(() => {
    statusRef.current = status;
  }, [status]);

  useEffect(() => {
    if (!settings) return;

    const handleToggle = async () => {
      try {
        if (statusRef.current === 'recording') {
          const result = await stopRecording();
          if (settings.auto_copy_to_clipboard && result.text) {
            await writeText(result.text);
          }
        } else if (statusRef.current === 'idle' || statusRef.current === 'completed' || statusRef.current === 'error') {
          await startRecording();
        }
      } catch (error) {
        console.error('Hotkey action failed:', error);
      }
    };

    const setupHotkeys = async () => {
      try {
        await register(settings.hotkey_toggle_record, handleToggle);
        console.log('Hotkey registered:', settings.hotkey_toggle_record);
      } catch (error) {
        console.error('Failed to register hotkey:', error);
      }
    };

    setupHotkeys();

    return () => {
      unregister(settings.hotkey_toggle_record).catch(console.error);
    };
  }, [settings?.hotkey_toggle_record, settings?.auto_copy_to_clipboard, startRecording, stopRecording]);
}
