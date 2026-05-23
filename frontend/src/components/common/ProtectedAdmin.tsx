import { Navigate, Outlet } from 'react-router-dom';

function ProtectedAdmin() {
  const token = localStorage.getItem('admin_token');

  if (!token) {
    return <Navigate to="/h7-admin" replace />;
  }

  return <Outlet />;
}

export default ProtectedAdmin;