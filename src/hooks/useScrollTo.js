import { useCallback } from 'react';

/**
 * Custom hook to smoothly scroll to a section by ID.
 */
export function useScrollTo() {
  const scrollTo = useCallback((sectionId) => {
    const element = document.getElementById(sectionId);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  }, []);

  return scrollTo;
}
