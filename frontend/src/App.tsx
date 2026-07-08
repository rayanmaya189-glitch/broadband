import { RouterProvider } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { HelmetProvider } from 'react-helmet-async';
import { useEffect } from 'react';
import { router } from './routes';
import { useAuthStore } from './store/authStore';
import { useTheme } from './hooks/useTheme';
import JsonLd from './components/seo/JsonLd';
import { loadWebVitals, monitorPageInteraction } from './utils/seoUtils';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 5 * 60 * 1000,
      refetchOnWindowFocus: false,
    },
  },
});

function AppContent() {
  const hydrate = useAuthStore((s) => s.hydrate);
  useTheme();

  useEffect(() => {
    hydrate();
    
    // Initialize SEO monitoring
    loadWebVitals();
    monitorPageInteraction();
  }, [hydrate]);

  return <RouterProvider router={router} />;
}

export default function App() {
  return (
    <HelmetProvider>
      <QueryClientProvider client={queryClient}>
        <JsonLd />
        <AppContent />
      </QueryClientProvider>
    </HelmetProvider>
  );
}
