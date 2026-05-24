import { useEffect, useState } from 'react';
import { Navigate, Outlet } from 'react-router-dom';
import { verifyAdminSession } from '../../services/api';

function ProtectedAdmin() {
  const [isChecking, setIsChecking] = useState(true);
  const [isAllowed, setIsAllowed] = useState(false);

  useEffect(() => {
    verifyAdminSession()
      .then((valid) => {
        if (!valid) {
          localStorage.removeItem('admin_token');
        }

        setIsAllowed(valid);
      })
      .catch(() => {
        localStorage.removeItem('admin_token');
        setIsAllowed(false);
      })
      .finally(() => {
        setIsChecking(false);
      });
  }, []);

  if (isChecking) {
    return <div className="state-message">Checking admin session...</div>;
  }

  if (!isAllowed) {
    return <Navigate to="/h7-admin" replace />;
  }

  return <Outlet />;
}

export default ProtectedAdmin;