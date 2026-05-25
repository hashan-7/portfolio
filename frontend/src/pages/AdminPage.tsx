import { useEffect, useState } from 'react';
import AdminDashboard from '../components/admin/AdminDashboard';
import AdminLogin from '../components/admin/AdminLogin';
import { logoutAdmin, verifyAdminSession } from '../services/api';

function AdminPage() {
  const [isChecking, setIsChecking] = useState(true);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  useEffect(() => {
    let active = true;

    verifyAdminSession()
      .then((valid) => {
        if (!active) {
          return;
        }

        if (valid) {
          setIsAuthenticated(true);
        } else {
          logoutAdmin();
          setIsAuthenticated(false);
        }
      })
      .catch(() => {
        if (!active) {
          return;
        }

        logoutAdmin();
        setIsAuthenticated(false);
      })
      .finally(() => {
        if (active) {
          setIsChecking(false);
        }
      });

    return () => {
      active = false;
    };
  }, []);

  if (isChecking) {
    return <div className="state-message">Checking admin session...</div>;
  }

  if (!isAuthenticated) {
    return <AdminLogin onLoginSuccess={() => setIsAuthenticated(true)} />;
  }

  return <AdminDashboard />;
}

export default AdminPage;