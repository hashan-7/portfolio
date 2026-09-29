import { lazy, Suspense } from 'react';
import { Navigate, Route, Routes } from 'react-router-dom';

const HomePage = lazy(() => import('./pages/HomePage'));
const AdminPage = lazy(() => import('./pages/AdminPage'));

function RouteFallback() {
  return (
    <main className="route-loading" aria-busy="true" aria-label="Loading page">
      <span className="route-loading-mark">H7</span>
      <span>Preparing experience</span>
    </main>
  );
}

function App() {
  return (
    <Suspense fallback={<RouteFallback />}>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/h7-admin" element={<AdminPage />} />
        <Route path="/h7-admin/dashboard" element={<Navigate to="/h7-admin" replace />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Suspense>
  );
}

export default App;
