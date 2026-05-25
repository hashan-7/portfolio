import { useEffect, useState } from 'react';
import { loginAdmin, verifyAdminSession } from '../../services/api';

interface AdminLoginProps {
  onLoginSuccess?: () => void;
}

function AdminLogin({ onLoginSuccess }: AdminLoginProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const [isChecking, setIsChecking] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    let active = true;

    verifyAdminSession()
      .then((valid) => {
        if (!active) {
          return;
        }

        if (valid) {
          onLoginSuccess?.();
        } else {
          localStorage.removeItem('admin_token');
        }
      })
      .catch(() => {
        if (active) {
          localStorage.removeItem('admin_token');
        }
      })
      .finally(() => {
        if (active) {
          setIsChecking(false);
        }
      });

    return () => {
      active = false;
    };
  }, [onLoginSuccess]);

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
      onLoginSuccess?.();
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