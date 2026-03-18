import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { WalletProvider } from './contexts/WalletContext';
import { ThemeProvider } from './contexts/ThemeContext';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import Markets from './pages/Markets';
import Borrow from './pages/Borrow';
import Supply from './pages/Supply';
import History from './pages/History';
import Governance from './pages/Governance';
import Settings from './pages/Settings';
import './App.css';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 3,
      staleTime: 10000,
      refetchOnWindowFocus: false,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <WalletProvider>
          <Router>
            <Layout>
              <Routes>
                <Route path="/" element={<Dashboard />} />
                <Route path="/markets" element={<Markets />} />
                <Route path="/borrow" element={<Borrow />} />
                <Route path="/supply" element={<Supply />} />
                <Route path="/history" element={<History />} />
                <Route path="/governance" element={<Governance />} />
                <Route path="/settings" element={<Settings />} />
              </Routes>
            </Layout>
          </Router>
        </WalletProvider>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
