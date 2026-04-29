import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { useState, useEffect } from 'react';
import { getMe } from './api/client';
import Landing from './pages/Landing';
import Login from './pages/Login';
import Signup from './pages/Signup';
import Dashboard from './pages/Dashboard';
import ConnectorConfig from './pages/ConnectorConfig';
import IntegrationCatalog from './pages/IntegrationCatalog';
import ConnectManagedHub from './pages/ConnectManagedHub';
import ApiKeys from './pages/ApiKeys';
import AuditLog from './pages/AuditLog';

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

  if (checking) {
    return (
      <div style={{ minHeight: '100vh', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#64748b', fontSize: '0.95rem' }}>
        Checking session…
      </div>
    );
  }
  return authenticated ? <>{children}</> : <Navigate to="/auth/login" replace />;
}

function AuthRoute({ children }: { children: React.ReactNode }) {
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

  if (checking) {
    return (
      <div style={{ minHeight: '100vh', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#64748b', fontSize: '0.95rem' }}>
        Checking session…
      </div>
    );
  }
  return authenticated ? <Navigate to="/dashboard" replace /> : <>{children}</>;
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Landing />} />
        <Route path="/auth/login" element={<AuthRoute><Login /></AuthRoute>} />
        <Route path="/auth/signup" element={<AuthRoute><Signup /></AuthRoute>} />
        <Route path="/dashboard" element={
          <ProtectedRoute>
            <Dashboard />
          </ProtectedRoute>
        } />
        <Route path="/connectors/catalog" element={
          <ProtectedRoute>
            <IntegrationCatalog />
          </ProtectedRoute>
        } />
        <Route path="/nango-catalog" element={
          <ProtectedRoute>
            <Navigate to="/connectors/catalog" replace />
          </ProtectedRoute>
        } />
        <Route path="/connectors/add-managed" element={
          <ProtectedRoute>
            <Navigate to="/connectors/catalog" replace />
          </ProtectedRoute>
        } />
        <Route path="/connectors/add-nango" element={
          <ProtectedRoute>
            <Navigate to="/connectors/catalog" replace />
          </ProtectedRoute>
        } />
        <Route path="/connectors/:platform/connect" element={
          <ProtectedRoute>
            <ConnectManagedHub />
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
        <Route path="/audit" element={
          <ProtectedRoute>
            <AuditLog />
          </ProtectedRoute>
        } />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}
