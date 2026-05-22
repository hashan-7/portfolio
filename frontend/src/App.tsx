import { useEffect, useState } from 'react';
import './App.css';
import Profile from './components/Profile';
import { getProfile } from './services/api';
import type { Profile as ProfileData } from './types';

function App() {
  const [profile, setProfile] = useState<ProfileData | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [errorMessage, setErrorMessage] = useState('');

  useEffect(() => {
    getProfile()
      .then((data) => {
        setProfile(data);
      })
      .catch((error: unknown) => {
        setErrorMessage(error instanceof Error ? error.message : 'Failed to load profile data.');
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  if (isLoading) {
    return <div className="state-message">Loading portfolio...</div>;
  }

  if (errorMessage) {
    return <div className="state-message error">{errorMessage}</div>;
  }

  if (!profile) {
    return <div className="state-message error">Profile data is not available.</div>;
  }

  return <Profile profile={profile} />;
}

export default App;