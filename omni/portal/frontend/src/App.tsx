import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { useState, useEffect } from 'react';
import { getMe } from './api/client';
import Login from './pages/Login';
import Dashboard from './pages/Dashboard';
import ConnectorConfig from './pages/ConnectorConfig';
import ApiKeys from './pages/ApiKeys';

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const [checking, setChecking] = useState(true);
  const [authenticated, setAuthenticated] = useState(false);

  useEffect(() => {
    getMe().then(me => {
      setAuthenticated(!!me);
      setChecking(false);
    }).catch(() => {
      setAuthenticated(false);
      setChecking(false);
    });
  }, []);

  if (checking) return null;
  return authenticated ? <>{children}</> : <Navigate to="/auth/login" replace />;
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/auth/login" element={<Login />} />
        <Route path="/" element={
          <ProtectedRoute>
            <Dashboard />
          </ProtectedRoute>
        } />
        <Route path="/connectors/:platform" element={
          <ProtectedRoute>
            <ConnectorConfig />
          </ProtectedRoute>
        } />
        <Route path="/api-keys" element={
          <ProtectedRoute>
            <ApiKeys />
          </ProtectedRoute>
        } />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}
