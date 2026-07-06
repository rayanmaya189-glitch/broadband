import { useMutation } from '@tanstack/react-query';
import { checkAvailability } from '../api/plans';

export function useAvailability() {
  return useMutation({
    mutationFn: (location: string) => checkAvailability(location),
  });
}
