import { useUIStore } from '../../store/uiStore';

export function toast(message: string, type: 'success' | 'error' | 'info' = 'success') {
  useUIStore.getState().addToast(message, type);
}
