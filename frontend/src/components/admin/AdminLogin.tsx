import { useState } from 'react';
import { loginAdmin, logoutAdmin } from '../../services/api';

interface AdminLoginProps {
  onLoginSuccess?: () => void;
}

function AdminLogin({ onLoginSuccess }: AdminLoginProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

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
      logoutAdmin();
      setErrorMessage(error instanceof Error ? error.message : 'Admin login failed.');
    } finally {
      setIsSubmitting(false);
    }
  };

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
              autoFocus
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

        {errorMessage && <p className="admin-error" role="alert">{errorMessage}</p>}

        <p className="admin-field-help">
          This route is hidden from the public UI, but real authentication is still required.
        </p>
      </section>
    </main>
  );
}

export default AdminLogin;
