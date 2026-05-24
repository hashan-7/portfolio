import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { loginAdmin, verifyAdminSession } from '../../services/api';

function AdminLogin() {
  const navigate = useNavigate();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const [isChecking, setIsChecking] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    verifyAdminSession()
      .then((valid) => {
        if (valid) {
          navigate('/h7-admin/dashboard', { replace: true });
        } else {
          localStorage.removeItem('admin_token');
        }
      })
      .catch(() => {
        localStorage.removeItem('admin_token');
      })
      .finally(() => {
        setIsChecking(false);
      });
  }, [navigate]);

  const handleLogin = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (!email.trim() || !password.trim()) {
      setErrorMessage('Please enter admin email and password.');
      return;
    }

    setIsSubmitting(true);
    setErrorMessage('');

    try {
      await loginAdmin(email.trim(), password);
      navigate('/h7-admin/dashboard', { replace: true });
    } catch (error) {
      localStorage.removeItem('admin_token');
      setErrorMessage(error instanceof Error ? error.message : 'Admin login failed.');
    } finally {
      setIsSubmitting(false);
    }
  };

  if (isChecking) {
    return <div className="state-message">Checking admin session...</div>;
  }

  return (
    <main className="admin-login-page">
      <section className="admin-login-card">
        <div className="admin-login-brand">
          <div className="admin-brand-logo">H7</div>
          <div>
            <p className="eyebrow">Private Admin</p>
            <h1>Portfolio Manager</h1>
          </div>
        </div>

        <p className="admin-muted">
          Sign in to manage profile data, projects, certificates, education, media paths, and
          chatbot context.
        </p>

        <form className="admin-login-form" onSubmit={handleLogin}>
          <label>
            Admin Email
            <input
              type="email"
              value={email}
              onChange={(event) => setEmail(event.target.value)}
              placeholder="admin@email.com"
              autoComplete="email"
            />
          </label>

          <label>
            Admin Password
            <input
              type="password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              placeholder="••••••••"
              autoComplete="current-password"
            />
          </label>

          <button type="submit" disabled={isSubmitting}>
            {isSubmitting ? 'Signing in...' : 'Sign In'}
          </button>
        </form>

        {errorMessage && <p className="admin-error">{errorMessage}</p>}

        <p className="admin-field-help">
          This route is hidden from the public UI, but real authentication is still required.
        </p>
      </section>
    </main>
  );
}

export default AdminLogin;