import { Navigate, Route, Routes } from 'react-router-dom';
import HomePage from './pages/HomePage';
import AdminPage from './pages/AdminPage';
import './App.css';

function App() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/h7-admin" element={<AdminPage />} />
      <Route path="/h7-admin/dashboard" element={<Navigate to="/h7-admin" replace />} />
      <Route path="*" element={<HomePage />} />
    </Routes>
  );
}

export default App;