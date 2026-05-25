import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import './App.css';
import AdminPage from './pages/AdminPage';
import HomePage from './pages/HomePage';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />} />

        {/* Hidden admin routes. No visible admin link is rendered in the public portfolio. */}
        <Route path="/h7-admin" element={<AdminPage />} />
        <Route path="/admin" element={<AdminPage />} />

        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
